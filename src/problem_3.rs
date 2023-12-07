
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIAGONAL_INDEX: Vec<(i32, i32)> = vec![
        (-1, -1), (-1, 0), (-1, 1), 
        (0, -1), /*(0, 0),*/ (0, 1), 
        (1, -1), (1, 0), (1, 1)
    ];
}

pub struct Problem3 {
    pub input_path: String
}   

type Row = Vec<Option<i32>>;
type Grid = Vec<Row>;

impl Problem3 {
    fn read_file(&self) -> String {
        std::fs::read_to_string(&self.input_path).unwrap()
    }

    fn build_grid(&self, with_repetition: bool) -> Grid {
        let number_regex = Regex::new(r"\d+").unwrap();
        let symbol_regex = Regex::new(r"[^.\d]").unwrap();
        
        self.read_file().lines().map(|line| {
            let mut row: Row = vec![None; line.len()];
            number_regex.find_iter(line).for_each(|_match|{
                let num = _match.as_str().parse::<i32>().unwrap();
                row[_match.start()] = Some(num);
                if with_repetition {
                    for i in _match.start()+1.._match.end(){ row[i] = Some(num) }
                }
            });

            symbol_regex.find_iter(line).for_each(|_match| {
                let value = match _match.as_str() {
                    "*" => Some(-1),
                    _ => Some(0)
                };
                for i in _match.start().._match.end(){ row[i] = value }
            });

            row
        }).collect()
    }

    fn get_diagonal_coords(&self, no_of_digits: usize, no_of_rows: usize, no_of_cols: usize, i: usize, j: usize) -> Vec<(i32, i32)> {
        (0..no_of_digits)
            .map(|k| k as i32)
            .flat_map(|k| DIAGONAL_INDEX.iter().map(move |(x, y)| (*x, *y+k)))
            .map(|(x, y)| (x+(i as i32), y+(j as i32)))
            .filter(|(x, y)| {
                (*x >=0) && 
                (*y >= 0) && 
                (*x < (no_of_rows as i32)) && 
                (*y < (no_of_cols as i32)) && 
                !((*x == (i as i32)) && (*y == (j as i32)))
            }).collect::<Vec<(i32, i32)>>()
    }

    fn build_mask(&self, grid: &Grid) -> Vec<Vec<bool>>{
        let mut mask = Vec::<Vec<bool>>::new();
        let no_of_rows = grid.len();


        for i in 0..no_of_rows {
            let col = &grid[i];
            let no_of_cols = col.len();
            mask.push(Vec::new());

            for j in 0..no_of_cols {
                let is_valid = match col[j] {
                    Some(v) =>{
                        let no_of_digits = format!("{}", v).len();
                        self.get_diagonal_coords(no_of_digits, no_of_rows, no_of_cols, i, j).iter()
                        .map(|(x, y)| grid[*x as usize][*y as usize])
                        .fold(false, |acc, cur| acc || cur.is_some())
                    },
                    None => false,
                };
                mask[i].insert(j, is_valid);
            }
        }
        mask

    }

    pub fn part_1(&self) -> u32 {
        let grid = self.build_grid(false);
        let mask = self.build_mask(&grid);
        
        (0..mask.len()).flat_map(|i| {
            (0..mask[i].len()).map(move |j| (i, j))
        })
        .filter(|(i, j)| mask[*i][*j] && match grid[*i][*j]{
            Some(c) => c > 0,
            None => false
        })
        .map(|(i, j)| grid[i][j].unwrap() as u32)
        .sum()
    }

    pub fn part_2(&self) -> u32 {
        let grid = self.build_grid(true);

        let no_of_rows = grid.len();
        let no_of_cols = grid[0].len();

        let res = (0..no_of_rows)
            .flat_map(|i| {(0..no_of_cols).map(move |j| (i, j))})
            .filter(|(i, j)| match grid[*i][*j]{Some(c) => c == -1, None => false})
            .map(|(i, j)| {
                let mut vecs = self.get_diagonal_coords(1, no_of_rows, no_of_cols, i, j)
                    .iter()
                    .filter(|(x, y)|grid[*x as usize][*y as usize].is_some())
                    .map(|(i, j)| grid[*i as usize][*j as usize].unwrap() as i32)
                    .collect::<Vec<i32>>();
                vecs.dedup();
                vecs
            })
            .filter(|x| x.len()==2)
            .map(|x| x.iter().fold(1, |acc, cur| acc*cur) as u32)
            .sum();

        res
    }

}
