#[derive(Debug,Clone)]
pub struct OsInfo{
    pub arch : String, 
    pub os : String,
}
impl OsInfo{
    pub fn new()->Self{
        let info = uname::uname().unwrap();
        OsInfo {
         os : info.sysname, 
         arch : info.machine,
        }
    }
}
impl Default for OsInfo {
    fn default() -> Self { 
        OsInfo::new()
     }
}


#[cfg(test)]
mod test {
    use crate::manifest::os_info::{OsInfo};
    #[test]
    fn basic_os_detection(){
        // if cfg!(windows) {
        //     println!("this is windows");
        // } else if cfg!(unix) {
        //     println!("this is unix alike");
        // }
        // use uname;
        // let info = uname::uname().unwrap();
        // println!("{:?}",info);
        // let sys_name = info.sysname; 
        // let machine = info.machine; 
        let i = OsInfo::new();
        assert_eq!(String::from("Linux"), i.os);
    }
}