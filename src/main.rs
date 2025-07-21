use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::{fmt, io};
use std::fmt::Display;

fn main() ->io::Result<()>  {

   let file:File = File::open("kickstart.k")?;
   let reader:BufReader<File> = io::BufReader::new(file);
   let mut event_array:Vec<Event> = Vec::with_capacity(32);

   for line_result in reader.lines() {
      let line:String = line_result?;
      let trimmed_line:&str = line.trim();

      create_executable_object(trimmed_line,&mut event_array);
      println!("Line: {}", trimmed_line);
   }
   
   for event in &event_array {
      println!("{}", event);
   }
   Ok(())
}

fn create_executable_object(commands:&str,event_array:&mut Vec<Event>){
 let parsable:bool = try_for_parse(commands);

 if(parsable){
    let mut tag:String = String::with_capacity(32);
    let mut parameter:String = String::with_capacity(32);
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
    event_array.push(Event{param:parameter,tag,command:other});
 }

}

fn try_for_parse(commands:&str) ->bool{
   if(commands.len() > 3 && commands.starts_with("@")){
      println!("parsable");
    return  true;
   }
   println!("not parsable");
   return  false;
}

fn trim_line(line:String)->String{
  line
}

fn ask(what_to_ask:String)->String{
   println!("{what_to_ask}");
   let mut user_responce:String=String::new();
   stdin().read_line(&mut user_responce).unwrap();
   user_responce.trim().to_string()
}


fn parse()->(){

}


struct Event{
   tag:String,
   param:String,
   command:String,
}



impl Display for Event {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(
         f,
         "Tag: {}, Param: {}, Command: {}",
         self.tag, self.param, self.command
      )
   }
}

#[derive(PartialEq)]
enum Flow {
   CollectTagAndParam ,
   CollectRest,
}