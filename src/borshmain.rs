use borsh::{BorshDeserialize,BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize, Debug,Clone)]
struct User{
    username: String,
    password: String
}


fn main() {
   
   let u  = User{
    username: String::from("Akshit"),
    password : String::from("123")
   };

   let mut v = Vec::new();

   let _ans = u.serialize(&mut v);

   print!("{:?}", v);
   
   let user = User::try_from_slice(&v).unwrap();
   print!("{} {}",user.username, user.password);
   

}
