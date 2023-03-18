use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::structures::Player;
use crate::structures::Enemy;
use crate::structures::Projectile;
use crate::structures::Lives;
use crate::structures::Liveup;
use crate::structures::Bulletup;
use crate::structures::Pointsymbol;


pub const WIDTH : u32 =  800;
pub const HEIGHT: u32 =  600;

pub fn render(canvas: &mut WindowCanvas,player : &Player, vec : &mut Vec<Enemy>,vec1 : &mut Vec<Projectile>, vec2 : &mut Vec<Lives>, vec3 : &mut Vec<Liveup>, 
    vec4 : &mut Vec<Bulletup>,points :u32, vec5 : &mut Vec<Pointsymbol>) -> Result<(), String> {  /*Result jest używany w przypadku, 
        gdy zachodzi zachodzi potrzeba poprawnego obłużenia wyjątku w przypadku,
        gdy funkcja zwróci nieporządaną wartość. Jest to typ enum z wariantami OK(T)
        które oznacza zwrócenie poprawnej wartości oraz Err(E) reprezentującją 
        błąd do obsłużenia*/ 
        if points<3 {
            canvas.set_draw_color(Color::RGB(214,182,253));
        }
        
        if points>=3 && points <6 {
            canvas.set_draw_color(Color::RGB(98, 13, 32));
        }
        
        if points>=6 {
            canvas.set_draw_color(Color::RGB(214,100,253));
        }
    canvas.clear();
        let texture_creator = canvas.texture_creator(); /*tworzenie tekstury, która następnie 
        zostanie zrenderowana*/
    /*********************************************************/
    let mut square_texture1: Texture =
        texture_creator.create_texture_target(None, player.t_size,
            player.t_size)
        .expect("Failed to create a texture"); /*tworzy teksturę i przypisuje ją do 
        obiektu o określonych wymiarach*/
      canvas.with_texture_canvas(&mut square_texture1, |texture|{ texture.set_draw_color(player.color); texture.clear();}).expect("Failed to color");  //koloruje przypisaną teksturę przy okazji dokonując odświerzenia

    let texture1 = &square_texture1;

    let screen_position1 = player.position + Point::new(0,0);  //ustawia koordynaty
    let screen_rect1 = Rect::from_center(screen_position1, player.size.width(), player.size.height()); 
    
   
    /*********************************************************/
    for i in vec{
    let mut square_texture: Texture =
        texture_creator.create_texture_target(None, i.t_size,
            i.t_size)
        .expect("Failed to create a texture");

      canvas.with_texture_canvas(&mut square_texture, |texture|{
      texture.set_draw_color(i.color);

      texture.clear();

    }).expect("Failed to color");  

    let texture = &square_texture;

    let screen_position = i.position + Point::new(0,0);
    let screen_rect = Rect::from_center(screen_position, i.size.width(), i.size.height());
    let c = canvas.copy(texture,i.size,screen_rect);
    match c{
        Ok(c) => c,
        Err(c) => return Err(c),
    }
}
    /*********************************************************/
    for i in vec1{
        let mut square_texture: Texture =
            texture_creator.create_texture_target(None, i.t_size,
                i.t_size)
            .expect("Failed to create a texture");
    
          canvas.with_texture_canvas(&mut square_texture, |texture|{
          texture.set_draw_color(i.color);
    
          texture.clear();
    
        }).expect("Failed to color");
    
        let texture = &square_texture;
    
        let screen_position = i.position + Point::new(0,0);
        let screen_rect = Rect::from_center(screen_position, i.size.width(), i.size.height());
        let c = canvas.copy(texture,i.size,screen_rect);
        match c{
            Ok(c) => c,
            Err(c) => return Err(c),
        }
    }
    /*********************************************************/
    for i in vec2{
        let mut square_texture: Texture =
            texture_creator.create_texture_target(None, i.t_size,
                i.t_size)
            .expect("Failed to create a texture");
    
          canvas.with_texture_canvas(&mut square_texture, |texture|{
          texture.set_draw_color(i.color);
    
          texture.clear();
    
        }).expect("Failed to color");
    
        let texture = &square_texture;
    
        let screen_position = i.position + Point::new(0,0);
        let screen_rect = Rect::from_center(screen_position, i.size.width(), i.size.height());
        let c = canvas.copy(texture,i.size,screen_rect);
        match c{
            Ok(c) => c,
            Err(c) => return Err(c),
        }
    }

    /*********************************************************/
    for i in vec3{
        let mut square_texture: Texture =
            texture_creator.create_texture_target(None, i.t_size,
                i.t_size)
            .expect("Failed to create a texture");
    
          canvas.with_texture_canvas(&mut square_texture, |texture|{
          texture.set_draw_color(i.color);
    
          texture.clear();
    
        }).expect("Failed to color");
    
        let texture = &square_texture;
    
        let screen_position = i.position + Point::new(0,0);
        let screen_rect = Rect::from_center(screen_position, i.size.width(), i.size.height());
        let c = canvas.copy(texture,i.size,screen_rect);
        match c{
            Ok(c) => c,
            Err(c) => return Err(c),
        }
    }
    /*********************************************************/
        
    for i in vec4{
        let mut square_texture: Texture =
            texture_creator.create_texture_target(None, i.t_size,
                i.t_size)
            .expect("Failed to create a texture");
    
          canvas.with_texture_canvas(&mut square_texture, |texture|{
          texture.set_draw_color(i.color);
    
          texture.clear();
    
        }).expect("Failed to color");
    
        let texture = &square_texture;
    
        let screen_position = i.position + Point::new(0,0);
        let screen_rect = Rect::from_center(screen_position, i.size.width(), i.size.height());
        let c = canvas.copy(texture,i.size,screen_rect);
        match c{
            Ok(c) => c,
            Err(c) => return Err(c),
        }
    }
    /*********************************************************/
    for i in vec5{
        let mut square_texture: Texture =
            texture_creator.create_texture_target(None, i.t_size,
                i.t_size)
            .expect("Failed to create a texture");
    
          canvas.with_texture_canvas(&mut square_texture, |texture|{
          texture.set_draw_color(i.color);
    
          texture.clear();
    
        }).expect("Failed to color");
    
        let texture = &square_texture;
    
        let screen_position = i.position + Point::new(0,0);
        let screen_rect = Rect::from_center(screen_position, i.size.width(), i.size.height());
        let _c = canvas.copy(texture,i.size,screen_rect)?;
        /*match _c{
            Ok(c) => c,
            Err(c) => return Err(c),  //obydwa rozwiązania można stosować zamiennie
        }*/
    }

    /*********************************************************/

        canvas.copy(&texture1, player.size, screen_rect1)?; //dodaje do prezentacji kolejną 
        //warstwę zawierającą informację o teksturze, rozmiarach obiektu oraz jego położeniu
        canvas.present();

        Ok(())
    }


    pub fn check_collision(pro: &Projectile, ene: &Enemy) -> bool {
        let left_a;
        let left_b;
        
        let right_a;
        let right_b;
        
        let top_a;
        let top_b;
        
        let bottom_a;
        let bottom_b;
        
        left_a = pro.position.x;
        right_a = pro.position.x + (pro.size.width() as i32);
        top_a = pro.position.y;
        bottom_a = pro.position.y + (pro.size.height() as i32);
    
        left_b = ene.position.x;
        right_b = ene.position.x + (ene.size.width() as i32);
        top_b = ene.position.y;
        bottom_b = ene.position.y + (ene.size.height() as i32);
    
        if bottom_a <= top_b 
        {
            return false;
        }
    
        if top_a >= bottom_b 
        {
            return false;
        }
    
        if right_a <= left_b
        {
            return false;
        }
    
        if left_a >= right_b
        {
            return false;
        }
    
        return true;
    }
    
    pub fn check_collision1(ene: &Player,pro: &Enemy) -> bool {
        let left_a;
        let left_b;
    
        let right_a;
        let right_b;
    
        let top_a;
        let top_b;
    
        let bottom_a;
        let bottom_b;
        
        left_a = pro.position.x;
        right_a = pro.position.x + (pro.size.width() as i32);
        top_a = pro.position.y;
        bottom_a = pro.position.y + (pro.size.height() as i32);
    
        left_b = ene.position.x;
        right_b = ene.position.x + (ene.size.width() as i32);
        top_b = ene.position.y;
        bottom_b = ene.position.y + (ene.size.height() as i32);
    
        if bottom_a <= top_b 
        {
            return false;
        }
    
        if top_a >= bottom_b 
        {
            return false;
        }
    
        if right_a <= left_b
        {
            return false;
        }
    
        if left_a >= right_b
        {
            return false;
        }
    
        return true;
    }
    
    pub fn check_collision2(ene: &Projectile, pro: &Liveup) -> bool {
        let left_a;
        let left_b;
    
        let right_a;
        let right_b;
    
        let top_a;
        let top_b;
    
        let bottom_a;
        let bottom_b;
        
        left_a = pro.position.x;
        right_a = pro.position.x + (pro.size.width() as i32);
        top_a = pro.position.y;
        bottom_a = pro.position.y + (pro.size.height() as i32);
    
        left_b = ene.position.x;
        right_b = ene.position.x + (ene.size.width() as i32);
        top_b = ene.position.y;
        bottom_b = ene.position.y + (ene.size.height() as i32);
    
        if bottom_a <= top_b 
        {
            return false;
        }
    
        if top_a >= bottom_b 
        {
            return false;
        }
    
        if right_a <= left_b
        {
            return false;
        }
    
        if left_a >= right_b
        {
            return false;
        }
    
        return true;
    }
    
    pub fn check_collision3(ene: &Projectile,pro: &Bulletup) -> bool {
        let left_a;
        let left_b;
    
        let right_a;
        let right_b;
    
        let top_a;
        let top_b;
    
        let bottom_a;
        let bottom_b;
        
        left_a = pro.position.x;
        right_a = pro.position.x + (pro.size.width() as i32);
        top_a = pro.position.y;
        bottom_a = pro.position.y + (pro.size.height() as i32);
    
        left_b = ene.position.x;
        right_b = ene.position.x + (ene.size.width() as i32);
        top_b = ene.position.y;
        bottom_b = ene.position.y + (ene.size.height() as i32);
    
        if bottom_a <= top_b 
        {
            return false;
        }
    
        if top_a >= bottom_b 
        {
            return false;
        }
    
        if right_a <= left_b
        {
            return false;
        }
    
        if left_a >= right_b
        {
            return false;
        }
    
        return true;
    }
    
    pub fn fall(v1: &mut Vec<Enemy>,v2: &mut Vec<Liveup>,v3: &mut Vec<Bulletup>,i:i32) {
        for a in 0..v1.len(){
            v1[a].position=v1[a].position.offset(0,i);
        }
        
        for a in 0..v2.len(){
            v2[a].position=v2[a].position.offset(0,i);
        }
        
        for a in 0..v3.len(){
            v3[a].position=v3[a].position.offset(0,i);
        }
        //ta funkcja nie zwraca wartości, nie zachodzi konieczność dodatkowego zabezpieczania jej
    }
    
    
    pub fn fall_faster(v1: &mut Vec<Enemy>,v2: &mut Vec<Liveup>,v3: &mut Vec<Bulletup>,i:i32){
        for a in 0..v1.len(){
            v1[a].position=v1[a].position.offset(0,i*2);
        }
        
        for a in 0..v2.len(){
            v2[a].position=v2[a].position.offset(0,i*2);
        }
        
        for a in 0..v3.len(){
            v3[a].position=v3[a].position.offset(0,i*2);
        }
    }
    
    pub fn create_projectile(pro: &mut Vec<Projectile>, her: &mut Player){
        let p = Projectile{
            position: Point::new(her.position.x,her.position.y),
            size : Rect::new(0,0,10,10),
            t_size: 32,
            color: Color::RGB(50,95,255),
        };
        pro.push(p);
    }
    
    pub fn create_super_projectile(pro: &mut Vec<Projectile>, her: &mut Player){
        let p = Projectile{
            position: Point::new(her.position.x,her.position.y),
            size : Rect::new(0,0,10,10),
            t_size: 32,
            color: Color::RGB(9,244,253),
        };
        pro.push(p);
    }
    
    pub fn add_live(vec: &mut Vec<Lives>, width: u32, height: u32){
        let l1 = Lives{
            position: Point::new(width as i32,height as i32),
            size : Rect::new(0,0,30,30),
            t_size: 32,
            color: Color::RGB(247,115,0),
        };
        vec.push(l1);
    }

    pub fn add_enemy(vec: &mut Vec<Enemy>, width: u32, height: u32){
        let p2 = Enemy{
            position: Point::new(width as i32,height as i32),
            size : Rect::new(0,0,30,30),
            t_size: 32,
            color: Color::RGB(255,164,90),
        };
        vec.push(p2);
    }
    
    pub fn add_lifeup(vec: &mut Vec<Liveup>, width: u32, height: u32){
        let l8 = Liveup{
            position: Point::new(width as i32,height as i32),
            size : Rect::new(0,0,30,30),
            t_size: 32,
            color: Color::RGB(241,29,149),
        };
        vec.push(l8);
    }

    pub fn add_bulletup(vec: &mut Vec<Bulletup>, width: u32, height: u32){
        let l9 = Bulletup{
            position: Point::new(width as i32,height as i32),
            size : Rect::new(0,0,30,30),
            t_size: 32,
            color: Color::RGB(9,244,253),
        };
        vec.push(l9);
    }