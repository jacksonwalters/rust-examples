fn dynamicArray(n: i32, queries: &[Vec<i32>]) -> Vec<i32> {
    /* initialize array of n empty arrays */
    let arr = vec![vec![0; 0]; n as usize];
    let answers : Vec<i32> = vec![0; n as usize];
    let lastAnswer = 0;
    for query in queries.iter(){
        let query_type = query[0];
        let x = query[1];
        let y = query[2];
        println!("{},{},{}",query_type,x,y);
        /* query 1 x y */
        if query_type == 1{
            let idx = (x^lastAnswer) % n;
            arr[idx].push(y);
            println!("query_1 {}",idx);
        }
        /* query 2 x y */
        if query_type == 2{
            let idx = (x^lastAnswer) % n;
            lastAnswer = arr[idx][y % arr[idx].len()];
            answers.push(lastAnswer);
            println!("query_2 {}",lastAnswer);
        }
    }
    
    return answers;
}