use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    // 读取接下来的n行到数组
    let mut input_for_bottle_rows: Vec<Vec<i32>> = Vec::new();

    println!("请输入n行数据: ");
    // 循环读取数据
    for line in std::io::stdin().lock().lines() {
        let tmp_line = line.expect("读取错误");
        if tmp_line.is_empty() {
            break;
        }
        println!("读取到一行数据: {}", tmp_line);
        let tmp_array: Vec<i32> = tmp_line
            .trim()
            .split(" ")
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
        input_for_bottle_rows.push(tmp_array);
    }
    println!("最终读取到的数据: {:?}", input_for_bottle_rows);
    if !check_array_valid(&input_for_bottle_rows) {
        println!("数组不合法");
        return;
    }
    println!("数组合法, 开始计算...");

    // 动态规划做DFS
    // 定义缓存
    let mut cache: HashMap<Vec<Vec<i32>>, Option<Vec<Vec<usize>>>> = HashMap::new();
    // 定义答案
    // let mut answer: Vec<String> = Vec::new();
    let answer = calculate_answer(&input_for_bottle_rows, &mut cache);
    println!("最终答案: {:?}", answer);
}

// 检查数组是否合法
fn check_array_valid(array: &Vec<Vec<i32>>) -> bool {
    if array.len() == 0 || array[0].len() == 0 {
        return false;
    }
    let row_len = array[0].len();
    for row in array {
        println!("检查行: {:?}", row);
        if row.len() != row_len {
            return false;
        }
    }
    true
}

// 计算
fn calculate_answer(
    array: &Vec<Vec<i32>>,
    cache: &mut HashMap<Vec<Vec<i32>>, Option<Vec<Vec<usize>>>>,
) -> Option<Vec<Vec<usize>>> {
    // 检查是否还存在未排序完成的瓶子
    if !check_if_still_have_unsorted_bottle(array) {
        return Some(Vec::new());
    }

    // 查看缓存中是否有当前瓶子的结果
    let sorted_array = array.clone();
    // 生成(index, value)元组
    let mut index_array: Vec<(usize, &Vec<i32>)> = sorted_array.iter().enumerate().collect();
    // 对元组value进行排序
    index_array.sort_by(|a, b| a.1.cmp(b.1));
    // 取出index 和 value
    let index_values: Vec<Vec<i32>> = index_array.iter().map(|x| x.1.clone()).collect();
    let index_key: Vec<usize> = index_array.iter().map(|x| x.0).collect();
    if let Some(cached_result) = cache.get(&index_values) {
        if cached_result.is_none() {
            return None;
        }
        let result_tmp = cached_result.as_ref().unwrap();
        return Some(
            result_tmp
                .iter()
                .map(|x| vec![index_key[x[0]], index_key[x[1]]])
                .collect(),
        );
    }

    // 没有的话就先将当前状态加入缓存，防止死循环
    cache.insert(index_values.clone(), None);

    // 没有结果时进行计算，先获取此时有可能的移动方式
    let possible_moves: Vec<Vec<usize>> = get_possible_moves(array);
    let mut best_result: Option<Vec<Vec<usize>>> = None;
    // 遍历所有方式，进行计算
    for move_ in possible_moves {
        let new_array: Vec<Vec<i32>> = move_warter(array.clone(), &move_);
        let _result = calculate_answer(&new_array, cache);
        if let Some(res) = _result {
            if best_result.is_none() || res.len() < best_result.as_ref().unwrap().len() {
                best_result = Some(res);
                best_result
                    .as_mut()
                    .unwrap()
                    .insert(0, vec![move_[0], move_[1]]);
            }
        }
    }
    if best_result.is_none() {
        cache.insert(index_values, None);
        return None;
    }
    // 1:将key转换回来
    let need_cache_key: Vec<Vec<usize>> = best_result
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| {
            vec![
                // 去 index_values 里找 x[0] (比如2) 在哪 -> 返回 0
                index_key.iter().position(|&v| v == x[0]).unwrap(),
                // 去 index_values 里找 x[1] (比如1) 在哪 -> 返回 2
                index_key.iter().position(|&v| v == x[1]).unwrap(),
            ]
        })
        .collect();
    cache.insert(index_values, Some(need_cache_key));
    best_result
}

// 检查是否还有未排序完成的瓶子
fn check_if_still_have_unsorted_bottle(array: &Vec<Vec<i32>>) -> bool {
    for row in array {
        let first_color = row[0];
        for &other_color in row {
            if other_color != first_color {
                return true;
            }
        }
    }
    false
}

// 获取可能的移动方式
fn get_possible_moves(array: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut possible_moves: Vec<Vec<usize>> = Vec::new();
    for m in 0..array.len() {
        // 已经为空时直接跳过
        let m_array = &array[m];
        if m_array[m_array.len() - 1] == 0 {
            continue;
        }
        for n in 0..array.len() {
            if m == n {
                continue;
            }
            // 已经为空时直接跳过
            let n_array: &Vec<i32> = &array[n];
            // 如果m的顶部颜色和n的顶部颜色相同或者n为空，则可以移动
            if n_array[0] == 0
                && (m_array[get_last_color_index(&m_array)]
                    == n_array[get_last_color_index(&n_array)]
                    || n_array[n_array.len() - 1] == 0)
            {
                possible_moves.push(vec![m, n]);
            }
        }
    }
    possible_moves
}

// 获取最后一个颜色的索引
fn get_last_color_index(array: &Vec<i32>) -> usize {
    for (i, &color) in array.iter().enumerate() {
        if color != 0 {
            return i;
        }
    }
    0
}

// 移动水
fn move_warter(mut array: Vec<Vec<i32>>, move_: &Vec<usize>) -> Vec<Vec<i32>> {
    let from_index = move_[0];
    let to_index = move_[1];
    // 计算移动水量
    let mut move_warter_count = 0;
    // 标识当前水颜色
    let mut warter_color = 0;
    // 标识起始瓶第几格有水
    let mut from_bottle_index = -1;
    // 计算起始水瓶有几个水可以移动
    for i in 0..array[from_index].len() {
        if array[from_index][i] == 0 {
            continue;
        } else if warter_color == 0 || warter_color == array[from_index][i] {
            if from_bottle_index == -1 {
                from_bottle_index = i as i32;
            }
            warter_color = array[from_index][i];
            move_warter_count += 1;
        } else {
            break;
        }
    }
    // 计算目标水瓶可以接收多少水
    let mut can_receive_count = 0;
    // 标识目标瓶第几格可以接水
    let mut to_bottle_index = -1;
    for i in 0..array[to_index].len() {
        if array[to_index][i] == 0 {
            can_receive_count += 1;
            to_bottle_index += 1;
        } else {
            break;
        }
    }
    // 实际移动水量
    let actual_move_count = if move_warter_count < can_receive_count {
        move_warter_count
    } else {
        can_receive_count
    };
    // 执行移动
    for _ in 0..actual_move_count {
        // 1: 从起始水瓶移除
        array[from_index][from_bottle_index as usize] = 0;
        from_bottle_index += 1;
        // 2: 放入目标水瓶
        array[to_index][to_bottle_index as usize] = warter_color;
        to_bottle_index -= 1;
    }
    array
}
