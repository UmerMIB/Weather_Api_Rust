use async_std::task;
use surf;

async fn fetch (url : &str)-> Result< String, surf::Exception >{
    surf::get ( url ).recv_string().await
}

async fn execute(){
    match fetch ("https://samples.openweathermap.org/data/2.5/weather?q=London,uk&appid=b6907d289e10d714a6e88b30761fae22").await{
        Ok (s) => println!("The weather  is {:#?}",s ),
        Err (s) => println!("Error occured {:#?} ",s ),
    }
}

fn main(){
    task::block_on(execute());
}