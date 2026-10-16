use bt_hci::cmd::le::*;
use bt_hci::controller::ControllerCmdSync;
use embassy_futures::join::join;
use embassy_time::Duration;
use trouble_host::prelude::*;

#[gatt_server(connections_max = 2)]
struct Server {
    battery: BatteryService,
}

#[gatt_service(uuid = service::BATTERY)]
struct BatteryService {
    #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, value = 42, permissions(encrypted))]
    level: u8,
}

pub async fn run<C>(controller: C)
where
    C: Controller
        + for<'t> ControllerCmdSync<LeSetExtAdvData<'t>>
        + ControllerCmdSync<LeClearAdvSets>
        + ControllerCmdSync<LeSetExtAdvParams>
        + ControllerCmdSync<LeSetAdvSetRandomAddr>
        + ControllerCmdSync<LeReadNumberOfSupportedAdvSets>
        + for<'t> ControllerCmdSync<LeSetExtAdvEnable<'t>>
        + for<'t> ControllerCmdSync<LeSetExtScanResponseData<'t>>,
{
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<DefaultPacketPool, 2, 6, 1> = HostResources::new();
    let stack = trouble_host::new(controller, &mut resources)
        .set_random_address(address)
        .build();
    let mut runner = stack.runner();
    let mut peripheral = stack.peripheral();

    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "TrouBLE addresses",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();
    let a = BdAddr::new([1, 2, 3, 4, 5, 0xC0]);
    let b = BdAddr::new([2, 2, 3, 4, 5, 0xC0]);
    let mut advertise = async |name: &'static str, addr: BdAddr| {
        let mut data = [0; 31];
        let len = AdStructure::encode_slice(
            &[
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                AdStructure::CompleteLocalName(name.as_bytes()),
            ],
            &mut data,
        )
        .unwrap();
        let sets = [AdvertisementSet {
            params: AdvertisementParameters {
                primary_phy: PhyKind::Le1M,
                secondary_phy: PhyKind::Le1M,
                interval_min: Duration::from_millis(160),
                interval_max: Duration::from_millis(160),
                ..Default::default()
            },
            data: Advertisement::ConnectableScannableUndirected {
                adv_data: &data[..len],
                scan_data: &[],
            },
            address: Some(addr),
        }];
        let mut handles = AdvertisementSet::handles(&sets);
        let advertiser = peripheral.advertise_ext(&sets, &mut handles).await.unwrap();
        info!("Connect to {} at {}", name, Address::new(AddrKind::RANDOM, addr));
        let conn = advertiser.accept().await.unwrap();
        assert_eq!(conn.adv_handle(), Some(AdvHandle::new(0)));
        assert_eq!(conn.local_address(), Some(addr));
        info!(
            "{} connected: local={}, peer={}, adv_set=0 (address checked)",
            name,
            Address::new(AddrKind::RANDOM, conn.local_address().unwrap()),
            conn.peer_address()
        );
        conn.with_attribute_server(&server).unwrap()
    };
    join(async { runner.run().await.unwrap() }, async {
        loop {
            let first = advertise("TrouBLE A", a).await;
            join(serve("A", &first, server.battery.level.handle()), async {
                let second = advertise("TrouBLE B", b).await;
                assert!(first.raw().is_connected(), "Keep A connected while connecting B");
                assert_eq!(first.raw().local_address(), Some(a));
                info!("PASS: A and B are connected with different local addresses after reusing set 0");
                serve("B", &second, server.battery.level.handle()).await;
            })
            .await;
            info!("Both disconnected; reconnect A then B to check connection-slot reuse");
        }
    })
    .await;
}

async fn serve(name: &str, conn: &GattConnection<'_, '_, DefaultPacketPool>, battery_handle: u16) {
    #[cfg(feature = "security")]
    {
        conn.raw().set_bondable(true).unwrap();
        info!("{} security start: known_bond={}", name, conn.raw().is_bonded_peer());
        conn.raw().request_security().unwrap();
    }
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { .. } => {
                info!("{} disconnected", name);
                return;
            }
            #[cfg(feature = "security")]
            GattConnectionEvent::PairingComplete { bond, .. } => {
                info!(
                    "{} security complete: new_bond={}, bonded_peer={}",
                    name,
                    bond.is_some(),
                    conn.raw().is_bonded_peer()
                )
            }
            #[cfg(feature = "security")]
            GattConnectionEvent::Encrypted { security_level, .. } => {
                info!(
                    "{} link encrypted: {:?}, bonded_peer={}",
                    name,
                    security_level,
                    conn.raw().is_bonded_peer()
                )
            }
            #[cfg(feature = "security")]
            GattConnectionEvent::PairingFailed(err) => panic!("Pairing failed: {:?}", err),
            GattConnectionEvent::Gatt { event } => {
                let battery_read = matches!(&event, GattEvent::Read(read) if read.handle() == battery_handle);
                event.accept().unwrap().send().await;
                if battery_read {
                    info!("{} Battery Level read served", name);
                }
            }
            _ => {}
        }
    }
}
