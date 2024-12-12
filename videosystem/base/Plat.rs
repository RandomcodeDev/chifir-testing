#[link(name = "Base")]
extern "C" {
    #[link_name = "?Plat_GetDateTime@@YAXAEAUDateTime@@_N@Z"]
    pub fn GetDateTime(time: &mut crate::Base::DateTime);
}
