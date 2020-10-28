//passenger templete structure
//enum seat type is OR function so we make an enum (selection between types)
//passenger initialize
//check seat type 
//check special request match function respective to seat type
#[derive(Debug)]
struct Passenger{
    name: String,
    id: u8,
    seat: Seattype,
    specialRequest:Option<String>
}
#[derive(Debug)]
enum Seattype{
    Economy,
    EconomyPlus,
    Business,
    Firstclass
}
impl Passenger{
    fn new(name:String, id:u8, seat:Seattype, specialRequest:Option<String>)->Passenger{
        Passenger{
            name,
            id,
            seat,
            specialRequest
        }
    }
}
fn CheckVIPtreatment(p:Passenger)->bool{
    match p.seat{
        Seattype::Business=> true,
        Seattype::Firstclass=> true,
        _=> false
    }
}
fn specialRequest(p:Passenger){
    match &p.specialRequest {
        Some(val)=>println!("passenger {:?} needs {:?}", p.name, &p.specialRequest),
        None=>println!("passenger {:?} dont have special request", p.name)
    }
}


fn main(){
let p1 = Passenger::new("Zain".to_string(), 3,Seattype::Business,Some("Ear plugs".to_string()));
println!("passenger 1 :{:?}", p1);
println!("check VIP treatment:{}",CheckVIPtreatment(p1));
}