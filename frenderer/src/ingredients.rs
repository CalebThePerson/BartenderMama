

pub enum Food {
    Lemon,
    Tequila,
    Wings,
    Ranch,
    FishSauce
}

pub struct Ingredient {
    pub ingredient: Food,
    pub good: Vec<Ingredients>,
    pub bad: Vec<Ingredients>,
}

impl Ingredient {

    pub fn new(ingredient: Food) -> Self {


            // let mut x_idx = 0.0 as f32;
            // let mut y_idx = 0.0 as f32;
            // let x_width = (16.0/32.0) as f32;
            // let y_width = (16.0/32.0) as f32;

        let good_vec = Vec::new();
        let bad_vec = Vec::new();

        match ingredient {   // Decide which sprite to use
            Food::Lemon =>      {(good_vec, bad_vec) = (vec![Tequila], vec![Wings, Ranch])},
            Food::Tequila =>       {(good_vec, bad_vec) = (vec![Lemon], vec![Tequila])},
            Food::Wings =>   {(good_vec, bad_vec) = (vec![Ranch], vec![Tequila, Lemon])},
            Food::Ranch =>   {(good_vec, bad_vec) = (vec![Wings], vec![Tequila, Lemon])},
            _ => ()
        }

        Self {
            ingredient: ingredient,
            good: good_vec,
            bad: bad_vec,
        }

    }


} 