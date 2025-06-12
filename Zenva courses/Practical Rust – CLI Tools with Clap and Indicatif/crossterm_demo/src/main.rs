use crossterm::{ExecutableCommand,QueueableCommand,terminal,cursor,style::{self,Stylize}};
use std::io::{self,Write};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All));

    for y in 0..40 {
        for x in 0..150 {
            if(y==0 || y==39) || (x==0 || x==149) {
                stdout
                    .queue(cursor::MoveTo(x,y))?
                    .queue(style::PrintStyledContent("‖".green()))?;
            }
        }
    }

    stdout.flush();
    
    Ok(())
}
