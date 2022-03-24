use rtlsdr::RTLSDRError;
use rtlsdr::RTLSDRDevice;
fn main() {
    let count = rtlsdr::get_device_count();
    println!("{}", count);
    let device: Result<RTLSDRDevice, RTLSDRError> = rtlsdr::open(0);
    assert_eq!(device.is_ok(),true);

    let mut result = match device {
        Ok(result) => result,
        Err(_err) => return ()
    };
    
    let device_strings = match result.get_usb_strings() {
        Ok(device_strings) => device_strings,
        Err(_err) => return ()
    };
    
    println!("{:?}", (device_strings.manufacturer, device_strings.product, device_strings.serial));
    let reset_result = result.reset_buffer();
    let was_reset = match reset_result {
        Ok(was_reset) => was_reset,
        Err(_err) => ()
    };
    println!("Device reset");
    loop {
        let stream = result.read_sync(16 * 16384);
        let data = match stream {
            Ok(data) => data,
            Err(_err) => return ()
        };
        if 128 - data[0] > 128 - data[1] || 128 - data[12] <  128 - data[13] {
            continue;
        }
        let pramble1 = 128 - data[1];
        let pramble2 = 128 - data[2];
        let pramble3 = 128 - data[3];
        let pramble4 = 128 - data[4];
        let pramble5 = 128 - data[5];
        let pramble6 = 128 - data[6];
        let pramble7 = 128 - data[7];
        let pramble8 = 128 - data[8];
        let pramble9 = 128 - data[9];
        let pramble10 = 128 - data[10];
        let pramble11 = 128 - data[11];
        let pramble12 = 128 - data[12];
        let pramble13 = 128 - data[13];
        println!("{:?}", [pramble1,pramble2,pramble3,pramble4,pramble5,pramble6,pramble7,pramble8,pramble9,pramble10,pramble11,pramble12,pramble13]);
    }

    println!("{}", count);
}
