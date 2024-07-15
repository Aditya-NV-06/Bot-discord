
use serenity::{
   async_trait, model::{channel::Message,gateway::Ready}, prelude::*
};

const WELCOME:&str=" 
  Hi welcome to the discord channel

  In this channel you can get all the 
  updates related to rust and computer science 
 
  Post in the <#1231492123971551253> channel 

  Have a nice a day
 ";

const  WORD :&str="hi";
struct Handler;
#[async_trait]
impl EventHandler for Handler{
    async fn message(&self,ctx:Context,msg:Message){
        if msg.content ==WORD {
              if let Err(why)=msg.channel_id.say(&ctx.http,WELCOME).await{
                   println!("Error in sending message: {:?}",why);
              }

        }
    }
   

    async fn ready(&self,_:Context,ready:Ready){ 
        println!("{} is connected!",ready.user.name);
    }
}

#[tokio::main]
async fn main() {
   let token="TOKEN";
   let intents = GatewayIntents::GUILD_MESSAGES
   | GatewayIntents::DIRECT_MESSAGES
   | GatewayIntents::MESSAGE_CONTENT;
   let mut client=Client::builder(&token,intents)
    .event_handler(Handler)
    .await
    .expect("Err creating cilent");

    if let Err(e)=client.start().await{
        println!("Cilent error : {:?}",e);
    }
}
  
   
