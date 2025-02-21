pub struct Mountain{
    pub height: f32,
    pub location: (f32,f32),
}

pub struct Tree{
    pub height: f32,
    pub location: (f32,f32),
}

pub struct River{
    pub length: f32,
    pub location: (f32,f32),
}

pub struct Obstacle{
    pub description: String,
    pub location: (f32,f32),
}

pub struct Environment {
 pub temperature: f32,
 pub pressure: f32,
 pub humidity: f32,
 pub wind_speed: f32,
 pub wind_direction: f32,
 pub mountains: Vec<Mountain>,
 pub trees: Vec<Tree>,
 pub rivers: Vec<River>,
 pub obstacles: Vec<Obstacle>,
}

impl Environment {
    pub fn new(temperature: f32, humidity: f32, pressure: f32, wind_speed: f32, wind_direction: f32, mountains:Vec<Mountain>, trees:Vec<Tree>,rivers:Vec<River>,obstacles: Vec<Obstacle>) -> Self {Environment {
    temperature,
    humidity,
    pressure,
    wind_speed,
    wind_direction,
    mountains,
    trees,
    rivers,
    obstacles,
    }
}
    
    pub fn update(&mut self, temperature: f32, humidity: f32, pressure: f32) {
    self.temperature = temperature;
    self.pressure = pressure;
    self.humidity = humidity;
    self.wind_speed = wind_speed;
    self.wind_direction = wind_direction;
    }

    pub fn display(&self){
        println!("Temperature: {} C", self.temperature);
        println!("Pressure: {} Pa", self.pressure);
        println!("Humidity: {} %", self.humidity);
        println!("Wind Speed: {} m/s", self.wind_speed);
        println!("Wind Direction: {} degrees", self.wind_direction);
        println!("Mountains: {:?}", self.mountains);
        println!("Trees: {:?}", self.trees);
        println!("Rivers: {:?}", self.rivers);
        println!("Obstacles: {:?}", self.obstacles);
    }

}
