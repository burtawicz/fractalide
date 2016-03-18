extern crate capnp;

#[macro_use]
extern crate rustfbp;

component! {
    DropIp,
    inputs( drop: any),
    inputs_array(),
    outputs(),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()> {
        let mut ip = try!(self.ports.recv("drop"));
        Ok(())
    }
}
