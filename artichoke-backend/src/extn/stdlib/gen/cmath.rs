use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "CMath", None)?;
    interp.0.borrow_mut().def_module::<CMath>(spec);
    
    
    
    interp.def_rb_source_file(
        b"cmath.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/cmath.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct CMath;


