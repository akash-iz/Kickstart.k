use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::{ io};
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
        _ => println!("tag not found {}",event.tag)
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
   let mut start_collecting_key:bool = false;
   let mut add_time:bool = false;

   //let mut need_white_space:bool = false;
   let mut result:String = String::with_capacity(32);
   let mut key:String =String::with_capacity(32);

   for ch in message.chars() {
      match (ch) {
         '$' => {

            if(start_collecting_key){
               key.push(ch);
            }else if(has_seen_dollar) {
               result.push(ch);
            }else{
               has_seen_dollar = true;
            }

         },
         '{' => {

            if(start_collecting_key){
               key.push(ch);
            }else if(has_seen_dollar) {
               start_collecting_key=true;
               has_seen_dollar = false;
            }else{
               result.push(ch);
            }

         },
         '}' => {

            if(start_collecting_key){
               start_collecting_key = false;
               add_time = true;
            } else{
               if has_seen_dollar {
                  result.push('$');
                  has_seen_dollar = false;
               }
               result.push(ch);
            }

         },

         _ => {
            if(start_collecting_key){
               key.push(ch);
            }else{
               if has_seen_dollar {
                  result.push('$');
                  has_seen_dollar = false;
               }
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
            result.push('{');
            result.push_str(&key);
            result.push('}');
         }

         key.clear();
         add_time = false;
      }

   }

   //after for loop last check
   if(start_collecting_key){
      result.push('$');
      result.push('{');
      result.push_str(&key);
   }
   //cannot true both anyway
   if(has_seen_dollar){
      result.push('$');
   }

   result
}


// event implementation
fn ask(event: Event, hash_map:&mut HashMap<String, String>)  {
   println!("\n{}",event.command);
   let mut user_responce:String=String::new();
   stdin().read_line(&mut user_responce).unwrap();
   hash_map.insert(event.param , user_responce.trim().to_string());
}


fn log(event: Event, hash_map: &mut HashMap<String, String>) {
   println!("{}\n",string_mapper(event.command,hash_map));
}

fn run(event: Event, hash_map: &mut HashMap<String, String>) {
   let command:String = string_mapper(event.command,hash_map);
   let parsed_para=string_mapper(event.param,hash_map);
   let parts: Vec<&str> = command.split_whitespace().collect();
   let mut cmd:Command;

   if (parts.len() > 0) {
      cmd = Command::new(parts[0]);
      cmd.args(&parts[1..]);

      if(parsed_para.len()>0){
         cmd.current_dir(parsed_para);
      }

      match cmd.status() {
         Ok(status) => println!("Process exited with: {}", status),
         Err(e) => eprintln!("Failed to run command: {}", e),
      }
   } else {
      eprintln!("No command provided!");
   }

}

