extern crate rand;  // biblioteka do generowania liczb pseudolosowych 
extern crate sdl2;  // biblioteka graficzna SDL2

mod structures; //podstawienie treści pliku structures.rs
mod modules;    //podstawienie treści pliku modules.rs

use sdl2::pixels::Color; 
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::{Duration};
use rand::Rng;

use crate::structures::Player;
use crate::structures::Lives;

/*pobieranie definicji poszczególnych klas z pliku structures */

fn main() -> Result<(), String> {
    let mut vec = Vec::new();
    let mut bul = Vec::new();
    let mut liv = Vec::new();
    let mut lifeup = Vec::new();
    let mut bulletup = Vec::new();
    let mut pointssymbols = Vec::new();

    /*
    utworzone zostały wektory do przechowywania zmiennych obiektów
    poszczególnych klas
    */


    let mut rng = rand::thread_rng(); /*zmienna mut. służąca do generowania 
    liczb pseudolosowych*/
    let sdl_context = sdl2::init()?; //inicjalizacja biblioteki
    let video_subsystem = sdl_context.video()?; //przypisanie cech biblioteki zmiennej 

    let mut hard_way = false; //poziom trudności
    let mut limit = false;    //limit żyć
    let mut n_bull = false;   //pocisk specjalny

    let mut i =0;
    let mut j =10;
    let mut ij =0;          //liczniki które zwiększają się co milisekundę
    let mut bullet_c = 0;   //licznik pocisków specjalnych
    let mut points = 0;     //licznik punktów

    let window = video_subsystem.window("Statek kosmiczny", modules::WIDTH, modules::HEIGHT)  //tworzenie okna 
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build() //inicjalizacja płaszczyzny gry
        .expect("could not make a canvas");
    
    let mut event_pump = sdl_context.event_pump()?;  //zbiera przerwania z klawiatury i dodaje je do pętli zdarzeń

        let mut p1 = Player{
            position: Point::new(60,550),
            size : Rect::new(0,0,30,30),
            t_size: 32,
            color: Color::RGB(255,255,0),
        };                                  //inicjalizacja gracza
        
        modules::add_live(&mut liv, modules::WIDTH-20,modules::HEIGHT-550);
        modules::add_live(&mut liv, modules::WIDTH-60,modules::HEIGHT-550);
        modules::add_live(&mut liv, modules::WIDTH-100,modules::HEIGHT-550); 
        
        //tworzenie oraz dodawnaie obiektów klasy lifeup do odpowiedniego jej kontenera

    

    'running: loop {   //nieskończona pętla "życia" gry
        
        if liv.len()==0 {
            print!("Porazka!\n");
            break 'running;
        }
        if points>=12 {
            print!("Zwyciestwo!\n");
            break 'running;
        }
        if liv.len()==5{
            limit=true;
        }
        if bullet_c==3 {
            bullet_c=0;
            n_bull=false;
        }

        ij+=1; 
        i+=1;
        j+=1; //liczniki zwiększają się co mikrosekundę

        if i>1{
            i-=1;
        }

        if j>30{
            j-=30;
        }

        if ij>600 {            
            ij-=600;
        }
                    //resetowanie liczników po osiągnięciu przez nie określonej wartości
        if j<2{
            modules::add_enemy(&mut vec,rng.gen_range(0 ,800),0);  //dodaje przeciwnika
        }

        if ij==550 {
            modules::add_lifeup(&mut lifeup,rng.gen_range(0 ,800),0);   //dodaje bonus #1
        }

        if(ij%5==0)&&(ij%30==0)&&(ij%20==0){
            modules::add_bulletup(&mut bulletup,rng.gen_range(0 ,800),0); //dodaje bonus #2
        }

        match hard_way{
            true =>  modules::fall(&mut vec,&mut lifeup,&mut bulletup,i), 
            false => modules::fall_faster(&mut vec,&mut lifeup,&mut bulletup,i), 
        }   //zastosowanie tzw. dopasowywania wzorców

        for event in event_pump.poll_iter() {  //pętla zdarzeń
            match event {
                
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::D),  .. } => {
                    p1.position=p1.position.offset(3,0);    
                },
                
                Event::KeyDown { keycode: Some(Keycode::A),  .. } => {
                    p1.position=p1.position.offset(-3,0);
                },//sterowanie
                Event::KeyDown { keycode: Some(Keycode::N), repeat: false, .. } => {
                    if hard_way==true {
                        hard_way=false;
                    }
                    else{
                        hard_way=true;
                    }
                },
                                //zmiana poziomu trudności
                Event::KeyDown { keycode: Some(Keycode::M), repeat: false, .. } => {
                        match n_bull{
                            false=> modules::create_projectile(&mut bul,&mut p1),
                            true => modules::create_super_projectile(&mut bul,&mut p1)
                        }
                },
                _ => {}
                
            }
        }

        for b in 0..bul.len(){
            bul[b].position=bul[b].position.offset(0,-3); //opadanie przeciwników
        }

        for b in 0..bul.len(){
            let _removed = vec.iter().position(|n| modules::check_collision(&bul[b],n)).map(|e| vec.remove(e)).is_some();
            let _removed1 = lifeup.iter().position(|n| modules::check_collision2(&bul[b],n)).map(|e| lifeup.remove(e)).is_some();
            let _removed2 = bulletup.iter().position(|n| modules::check_collision3(&bul[b],n)).map(|e| bulletup.remove(e)).is_some();

            if _removed {
                points+=1;
            }
            if  _removed1 && limit==false  {
                let l4 = Lives{
                    position: Point::new(liv.last().unwrap().position.x-40,50),
                    size : Rect::new(0,0,30,30),
                    t_size: 32,
                    color: Color::RGB(247,115,0),
                };
                liv.push(l4); 
            }

            if _removed && n_bull==true {
                bullet_c+=1
            }

            if _removed2 {
                n_bull = true;
            }
        }

        for b in 0..vec.len(){
            if modules::check_collision1(&p1,&vec[b]) {
                let _removed = vec.iter().position(|n| modules::check_collision1(&p1,n)).map(|e| vec.remove(e)).is_some();
                liv.pop();
                break;
            }
    }
        let r= modules::render(&mut canvas, &mut p1, &mut vec, &mut bul, &mut liv, &mut lifeup, &mut bulletup, points, &mut pointssymbols);
        match r{
            Ok(r)=>r,
            Err(e)=> return Err(e),
        }
        //renderowanie tego, co dzieje się w grze 
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); //regulacja liczby klatek na sekundę 
    }

    Ok(())
}
