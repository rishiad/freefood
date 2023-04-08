// use tokio::sync::Notify;


// trait Commands{
//     fn keyword(&self) -> &str;
//     fn execute(&self, args: Vec<String>);
// }


// pub async fn run_commands(notify: Notify) {
//     let commands = vec![
//         // Add commands here
//     ];
//     loop {
//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).expect("Failed to read command");
//         let mut args = input.split_whitespace();
//         let keyword = args.next().unwrap();
//         let args : Vec<String> = args.map(|s| s.to_string()).collect();
//         for command in &commands {
//             if command.keyword() == keyword {
//                 command.execute(args.to_vec());
//             }
//         }
//     }
// }