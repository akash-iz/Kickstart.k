use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::{fmt, io};
use std::collections::HashMap;
use std::process::Command;

fn main() ->io::Result<()>  {
   let file:File = File::open("kickstart.k")?;
   let reader:BufReader<File> = io::BufReader::new(file);
   let mut event_object:Vec<Event> = Vec::with_capacity(32);

   // this for loop here do parse the kickstart.k and make a object from it
   for line_result in reader.lines() {
      let line:String = line_result?;
      let trimmed_line:&str = line.trim();
      create_executable_object(trimmed_line,&mut event_object);
   }
   // this runs the command
   kick_start(event_object);

   press_any_key_to_exit();
   Ok(())
}

fn create_executable_object(commands:&str, event_object:&mut Vec<Event>){
 let parsable:bool = able_to_parse(commands);

 if(parsable){
    let mut tag:String = String::with_capacity(32);
    let mut parameter:String = String::with_capacity(0);
    let mut other:String = String::with_capacity(32);
    let mut has_seen_colon:bool = false;
    let mut current_flow:Flow = Flow::CollectTagAndParam;

    for ch in commands.chars() {
       if  (current_flow == Flow::CollectTagAndParam) {
          match (ch) {
             ':' => {
                has_seen_colon = true;
             },
             ' ' => {
                current_flow = Flow::CollectRest;
             },
             _ => {
                  if(has_seen_colon){
                     parameter.push(ch)
                  }else{
                     tag.push(ch);
                  }
             }
          }

       }else if (current_flow == Flow::CollectRest) {
            other.push(ch);
       }
    }
    event_object.push(Event{param:parameter,tag:match_tag(tag),command:other});
 }

}

fn able_to_parse(commands:&str) ->bool{
   if(commands.len() > 3 && commands.starts_with("@")){
    return  true;
   }
   return  false;
}




fn press_any_key_to_exit(){
   println!("\nPress enter to exit!");
   let mut user_responce:String=String::new();
   stdin().read_line(&mut user_responce).unwrap();
}

fn kick_start(event_object: Vec<Event>){
   let mut hash_map: HashMap<String, String> = HashMap::new();
   for event in event_object {
      event_runner(event,&mut hash_map);
   }
}

fn event_runner(event:Event,hash_map:&mut HashMap<String, String>){
     match event.tag {
        "@ask" => {
           ask(event,hash_map);
        },
        "@log" => {
           log(event,hash_map);
        },
        "@run" => {
           run(event,hash_map);
        },
        _ => todo!()
     }
}


struct Event{
   tag:&'static str,
   param:String,
   command:String,
}




#[derive(PartialEq)]
enum Flow {
   CollectTagAndParam ,
   CollectRest,
}


fn match_tag(tag: String) -> &'static str {
   match tag.as_str() {
      "@ask" => "@ask",
      "@log" => "@log",
      "@run"=>"@run",
      "location"=>"location",
      _ => "nil",
   }
}

fn string_mapper(message: String, hash_map: &mut HashMap<String, String>)->String{
   let mut has_seen_dollar:bool = false;
   let mut add_time:bool = false;
   let mut need_white_space:bool = false;
   let mut result:String = String::with_capacity(32);
   let mut key:String =String::with_capacity(32);

   for ch in message.chars() {
      match (ch) {
         '$' => {
            if(has_seen_dollar){
               add_time = true;
            }else{
               has_seen_dollar = true;
            }
         },
         ' ' => {
            if(has_seen_dollar){
               add_time = true;
               has_seen_dollar = false;
               need_white_space=true;
            }else{
               result.push(ch);
            }
         },
         _ => {
            if(has_seen_dollar){
               key.push(ch)
            }else{
               result.push(ch);
            }
         }
      }

      if(add_time){
         if(key.len() > 0 && hash_map.contains_key(&key)){
            let value= hash_map.get(&key).unwrap();
            result.push_str(value);
         }else {
            result.push('$');
            result.push_str(&key);
         }
         if(need_white_space){
            result.push(' ');
         }

         need_white_space=false;
         key.clear();
         add_time = false;
      }

   }
   if(has_seen_dollar){
      if(key.len() > 0 && hash_map.contains_key(&key)){
         let value= hash_map.get(&key).unwrap();
         result.push_str(value);
      }else {
         result.push('$');
         result.push_str(&key);
      }
   }

   result
}


// event implementation
fn ask(event: Event, hash_map:&mut HashMap<String, String>)  {
   println!("{}",event.command);
   let mut user_responce:String=String::new();
   stdin().read_line(&mut user_responce).unwrap();
   hash_map.insert(event.param , user_responce.trim().to_string());
}


fn log(event: Event, hash_map: &mut HashMap<String, String>) {
   println!("{}",string_mapper(event.command,hash_map));
}

fn run(event: Event, hash_map: &mut HashMap<String, String>) {
   let command:String = string_mapper(event.command,hash_map);

   let parts: Vec<&str> = command.split_whitespace().collect();
   let mut cmd:Command;

   if (parts.len() > 0) {
      cmd = Command::new(parts[0]);
      cmd.args(&parts[1..]);

      if(hash_map.contains_key(&event.param)){
         cmd.current_dir(hash_map.get(&event.param).unwrap());
      }

      match cmd.status() {
         Ok(status) => println!("Process exited with: {}", status),
         Err(e) => eprintln!("Failed to run command: {}", e),
      }
   } else {
      eprintln!("No command provided!");
   }

}

