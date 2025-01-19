
#![crate_type="dylib"]

#[repr(C)]
pub struct MacroContext {
    pub ident: String,
    pub tokens: String,
}
       
        
#[no_mangle]
fn process_macro(mac: &mut MacroContext) -> bool {
            if mac . ident == "http_get" { mac . tokens . push_str (", cx") ; return true ; } if mac . ident == "http_post" { mac . tokens . push_str (", cx") ; return true ; } if mac . ident == "http_put" { mac . tokens . push_str (", cx") ; return true ; } if mac . ident == "http_delete" { mac . tokens . push_str (", cx") ; return true ; } if mac . ident == "http_patch" { mac . tokens . push_str (", cx") ; return true ; }
            return false;
        }
        