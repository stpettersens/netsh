// Spoof netsh command to test SSID library on Appveyor CI.
fn main() {
    println!("Name : DUMMY_WIFI");
    println!("State : connected");
    println!("SSID : DUMMY_ID");
    println!("Profile : DUMMY_ID");
}
