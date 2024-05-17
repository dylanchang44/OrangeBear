
//return vector of pair (price,share)
pub fn flathat_calc(position: u32, start: u32, dip: u32, res:u32) -> Vec<(u32,u32)>{
    let mut distrib_vec: Vec<(u32,u32)>=vec![];
    let price_per_share=(start+dip)/2;
    let total_share=position/price_per_share;
    let share_per_step=total_share/res;
    let step_diff=(start-dip)/(res-1);
    for i in 0..res{
        distrib_vec.push((start-step_diff * i, share_per_step));
    }
    distrib_vec
}

pub fn pyramid_calc(position: u32, start: u32, dip: u32, res:u32) -> Vec<(u32,u32)>{
    let mut distrib_vec: Vec<(u32,u32)>=vec![];
    let position_per_step=position/res;
    let mut position_left=position;
    let step_diff=(start-dip)/(res-1);
    for i in 0..res{
        let price = start-step_diff * i;
        let share= position_per_step/price;
        distrib_vec.push((price, share));
        position_left-=price * share;
    }
    //un tested feature
    let end: usize = (res-1).try_into().unwrap();
    let mut tail = end;
    //println!("redundant calc start");
    while position_left>=distrib_vec[tail].0 {
        distrib_vec[tail].1 += 1;
        position_left-=distrib_vec[tail].0;
        //println!("redundant spent: inx = {}", tail);
        if tail==0 {tail=end;}
        else {tail-=1;}
    }
    distrib_vec
}
