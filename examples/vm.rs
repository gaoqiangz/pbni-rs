use pbni::{pbx::*, prelude::*};

fn main() {
    let vm = VM::new(r#"C:\Program Files (x86)\Appeon\Shared\PowerBuilder\PBVM190.DLL"#).unwrap();
    let session = vm.new_session("pbrs", &[r#"pbrs\pbw\pbrs.pbl"#]).unwrap();
    let rv: String = session.invoke_function("gf_pbtest", pbx_args!["test vm"]).unwrap();
    println!("rv: {}", rv);
    let rv: pbint = session.invoke_function(("MessageBox", "ISS"), pbx_args!["title", "content"]).unwrap();
    println!("rv: {}", rv);
}
