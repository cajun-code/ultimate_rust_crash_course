use std::{error::Error, io, thread, time::Duration};
use std::sync::mpsc;
use std::time::Instant;
use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show}, ExecutableCommand, event::{Event, KeyCode, self}};
use rusty_audio::Audio;
use invaders::{frame};
use invaders::frame::{Drawable, new_frame};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::render::render;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move","sounds/move.wav");
    audio.add("pew","sounds/pew.wav");
    audio.add("startup","sounds/startup.wav");
    audio.add("win","sounds/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop{
            let cur_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &cur_frame, false);
            last_frame = cur_frame;
        }
    });
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    // Game Loop
    'gameloop: loop{
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut cur_frame = new_frame();
        // input
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter =>{
                        if player.shoot(){
                            audio.play("pew")
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        player.update(delta);
        if invaders.update(delta){
            audio.play("move")
        }
        if player.detect_hits(&mut invaders){
            audio.play("explode");
        }
        // Draw & render

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables{
            drawable.draw(&mut cur_frame);
        }
        let _ = render_tx.send(cur_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose
        if invaders.all_killed(){
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom(){
            audio.play("lose");
            break 'gameloop;
        }
    }
    //cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
