use std::collections::{HashMap, HashSet};
use itertools::Itertools;

struct Data
{
    connections  :HashSet<(String,String)>,
    comp:HashSet<String>,
}

impl Data 
{    
    fn new(i:&[String])->Self
    {
        let mut connections   =  HashSet::new();
        let mut comp:HashSet<String>=  HashSet::new();

        for s in i
        {
            let s = s.split("-").collect::<Vec<&str>>();            
            let a = s[0].to_string();
            let b = s[1].to_string();
            comp.insert(a.clone());
            comp.insert(b.clone());
            connections.insert((a.clone(),b.clone()));
            connections.insert((b.clone(),a.clone()));
        }

        Self
        {
            connections,
            comp
        }
    }

    fn con(&self,a:&String,b:&String)->bool
    {
        self.connections.contains(&(a.clone(),b.clone())) 
    }

    fn count3(&self)->usize
    {
        let mut res = HashSet::new();

        for edge in self.connections.clone()
        {
            let a = edge.0;
            let b = edge.1;

            for c in self.comp.clone()
            {
                if  self.con(  &a,&c) && 
                    self.con(&b,&c)
                {
                    let mut v = [a.clone(),b.clone(),c.clone()];
                    v.sort();

                    //let key = [a,b,c].iter().sorted().collect::<Vec<&String>>();
                    res.insert((v[0].clone(),v[1].clone(),v[2].clone()));
                }
            }
        }

        let mut v = res.iter().collect::<Vec<&(String,String,String)>>();
        v.sort();
       
        v.iter()
         .filter(|(a,b,c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t') )
         .count()        
    }

    fn largest_clique(&self) -> Vec<String> 
    {
        let nodes: Vec<String> = self.comp.iter().cloned().collect();
        let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();

        for n in &nodes 
        {
            adjacency.insert(n.clone(), HashSet::new());
        }
        for (a,b) in &self.connections 
        {
            adjacency.get_mut(a).unwrap().insert(b.clone());
        }

        let mut best_clique = Vec::new();

        fn bron_kerbosch(
            r: &mut Vec<String>,
            p: &mut Vec<String>,
            x: &mut Vec<String>,
            adjacency  : &HashMap<String, HashSet<String>>,
            best_clique: &mut Vec<String>) 
        {
            if p.is_empty() && x.is_empty() 
            {
                if r.len() > best_clique.len() 
                {
                    *best_clique = r.clone();
                }
                return;
            }

            let pivot = p.first().cloned().unwrap_or_default();

            if !adjacency.contains_key(&pivot) { return; }
            let neighbors = adjacency.get(&pivot).unwrap();

            let mut candidates = Vec::new();
            for v in p.iter() 
            {
                if !neighbors.contains(v) { candidates.push(v.clone()); }
            }

            for v in candidates 
            {
                r.push(v.clone());

                let adj_v = adjacency.get(&v).unwrap();
                
                let new_p: Vec<String> = p.iter().filter(|&n| adj_v.contains(n)).cloned().collect();
                let new_x: Vec<String> = x.iter().filter(|&n| adj_v.contains(n)).cloned().collect();

                bron_kerbosch(r, &mut new_p.clone(), &mut new_x.clone(), adjacency, best_clique);

                r.pop();
                p.retain(|n| n != &v);
                x.push(v);
            }
        }

        bron_kerbosch(&mut Vec::new(),&mut nodes.clone(),&mut Vec::new(), &adjacency, &mut best_clique);

        best_clique
    }

    fn count_max(&self)->String
    {
        self.largest_clique().iter().sorted().join(",")
    }
    
}

pub fn part1(data:&[String])->usize
{    
    Data::new(data).count3()    
}

pub fn part2(data:&[String])->String
{
    Data::new(data).count_max()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day23");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "kh-tc".to_string(),
        "qp-kh".to_string(),
        "de-cg".to_string(),
        "ka-co".to_string(),
        "yn-aq".to_string(),
        "qp-ub".to_string(),
        "cg-tb".to_string(),
        "vc-aq".to_string(),
        "tb-ka".to_string(),
        "wh-tc".to_string(),
        "yn-cg".to_string(),
        "kh-ub".to_string(),
        "ta-co".to_string(),
        "de-co".to_string(),
        "tc-td".to_string(),
        "tb-wq".to_string(),
        "wh-td".to_string(),
        "ta-ka".to_string(),
        "td-qp".to_string(),
        "aq-cg".to_string(),
        "wq-ub".to_string(),
        "ub-vc".to_string(),
        "de-ta".to_string(),
        "wq-aq".to_string(),
        "wq-vc".to_string(),
        "wh-yn".to_string(),
        "ka-de".to_string(),
        "kh-ta".to_string(),
        "co-tc".to_string(),
        "wh-qp".to_string(),
        "tb-vc".to_string(),
        "td-yn".to_string(),
    ];
    assert_eq!(part1(&v),7);
}


#[test]
fn test2()
{
    let v = vec![
        "kh-tc".to_string(),
        "qp-kh".to_string(),
        "de-cg".to_string(),
        "ka-co".to_string(),
        "yn-aq".to_string(),
        "qp-ub".to_string(),
        "cg-tb".to_string(),
        "vc-aq".to_string(),
        "tb-ka".to_string(),
        "wh-tc".to_string(),
        "yn-cg".to_string(),
        "kh-ub".to_string(),
        "ta-co".to_string(),
        "de-co".to_string(),
        "tc-td".to_string(),
        "tb-wq".to_string(),
        "wh-td".to_string(),
        "ta-ka".to_string(),
        "td-qp".to_string(),
        "aq-cg".to_string(),
        "wq-ub".to_string(),
        "ub-vc".to_string(),
        "de-ta".to_string(),
        "wq-aq".to_string(),
        "wq-vc".to_string(),
        "wh-yn".to_string(),
        "ka-de".to_string(),
        "kh-ta".to_string(),
        "co-tc".to_string(),
        "wh-qp".to_string(),
        "tb-vc".to_string(),
        "td-yn".to_string(),
    ];
    assert_eq!(part2(&v),"co,de,ka,ta");
}
