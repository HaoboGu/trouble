extern crate std;

use core::cell::RefCell;
use core::convert::Infallible;
use core::future::Future;
use std::rc::Rc;
use std::vec::Vec;

use bt_hci::cmd::le::LeReadNumberOfSupportedAdvSets;
use bt_hci::cmd::{self, AsyncCmd, Cmd, CmdReturnBuf, Opcode, SyncCmd};
use bt_hci::controller::{ControllerCmdAsync, ControllerCmdSync};
use bt_hci::FromHciBytes;

pub struct MockController {}

impl MockController {
    pub fn new() -> Self {
        Self {}
    }
}

impl embedded_io::ErrorType for MockController {
    type Error = Infallible;
}

impl bt_hci::controller::blocking::Controller for MockController {
    type Buffer<'a> = [u8; 259];

    fn alloc_buf(&self) -> Result<Self::Buffer<'_>, Self::Error> {
        Ok([0u8; 259])
    }

    fn write_acl_data(&self, packet: &bt_hci::data::AclPacket) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_sync_data(&self, packet: &bt_hci::data::SyncPacket) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_iso_data(&self, packet: &bt_hci::data::IsoPacket) -> Result<(), Self::Error> {
        todo!()
    }

    fn try_write_acl_data(
        &self,
        packet: &bt_hci::data::AclPacket,
    ) -> Result<(), bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }

    fn try_write_sync_data(
        &self,
        packet: &bt_hci::data::SyncPacket,
    ) -> Result<(), bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }

    fn try_write_iso_data(
        &self,
        packet: &bt_hci::data::IsoPacket,
    ) -> Result<(), bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }

    fn read<'a>(&self, buf: &'a mut Self::Buffer<'_>) -> Result<bt_hci::ControllerToHostPacket<'a>, Self::Error> {
        todo!()
    }

    fn try_read<'a>(
        &self,
        buf: &'a mut Self::Buffer<'_>,
    ) -> Result<bt_hci::ControllerToHostPacket<'a>, bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }
}

impl bt_hci::controller::Controller for MockController {
    type Buffer<'a> = [u8; 259];

    fn alloc_buf(&self) -> Result<Self::Buffer<'_>, Self::Error> {
        Ok([0u8; 259])
    }

    fn write_acl_data(&self, packet: &bt_hci::data::AclPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn write_sync_data(&self, packet: &bt_hci::data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn write_iso_data(&self, packet: &bt_hci::data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn read<'a>(
        &self,
        buf: &'a mut Self::Buffer<'_>,
    ) -> impl Future<Output = Result<bt_hci::ControllerToHostPacket<'a>, Self::Error>> {
        async { todo!() }
    }
}

impl<C: SyncCmd> ControllerCmdSync<C> for MockController {
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, cmd::Error<Self::Error>>> {
        async { todo!() }
    }
}

impl<C: AsyncCmd> ControllerCmdAsync<C> for MockController {
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), cmd::Error<Self::Error>>> {
        async { todo!() }
    }
}

/// Records the opcode of every command and answers it with zeroed return parameters, except for
/// reporting 4 supported advertising sets.
pub struct RecordingController {
    pub sent: Rc<RefCell<Vec<Opcode>>>,
}

impl embedded_io::ErrorType for RecordingController {
    type Error = Infallible;
}

impl bt_hci::controller::Controller for RecordingController {
    type Buffer<'a> = [u8; 259];

    fn alloc_buf(&self) -> Result<Self::Buffer<'_>, Self::Error> {
        Ok([0u8; 259])
    }

    fn write_acl_data(&self, packet: &bt_hci::data::AclPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn write_sync_data(&self, packet: &bt_hci::data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn write_iso_data(&self, packet: &bt_hci::data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn read<'a>(
        &self,
        buf: &'a mut Self::Buffer<'_>,
    ) -> impl Future<Output = Result<bt_hci::ControllerToHostPacket<'a>, Self::Error>> {
        async { todo!() }
    }
}

impl<C: SyncCmd> ControllerCmdSync<C> for RecordingController {
    fn exec(&self, _cmd: &C) -> impl Future<Output = Result<C::Return, cmd::Error<Self::Error>>> {
        self.sent.borrow_mut().push(C::OPCODE);
        let mut buf = C::ReturnBuf::new();
        if C::OPCODE == LeReadNumberOfSupportedAdvSets::OPCODE {
            buf.as_mut()[0] = 4;
        }
        let ret = C::Return::from_hci_bytes(buf.as_ref()).unwrap().0;
        async move { Ok(ret) }
    }
}

impl<C: AsyncCmd> ControllerCmdAsync<C> for RecordingController {
    fn exec(&self, _cmd: &C) -> impl Future<Output = Result<(), cmd::Error<Self::Error>>> {
        self.sent.borrow_mut().push(C::OPCODE);
        async { Ok(()) }
    }
}
