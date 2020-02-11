mod graphics
{
    #[derive(Copy)]
    struct Point
    {
        red: u32,
        blue: u32,
        green: u32,
    };

    pub struct PPMImg
    {
        height: u32,
        width: u32,
        depth: u32,
        data: Vec<Vec<Point>>,
    };

    impl PPMImg
    {
        pub fn new(height, width, depth)-> self
        {
            let mut img = PPMImg
            {
                height,
                width,
                depth
            };
            
        }
    }
}
