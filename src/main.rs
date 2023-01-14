fn main() {

    fn ideal_speed(distance: f32, angle: f32) -> i32 {
        // constants that represent the car's maximum speed, minimum speed, threshold distance and angle
        let max_speed = 100.0;
        let min_speed = 32.0;
        let threshold_distance = 1000.0;
        let threshold_angle = 16.0;
        let max_angle = 128.0;
    
        // calculate the ideal speed based on the distance and angle to the target
        let speed;
        if distance >= threshold_distance || (angle.abs() <= threshold_angle) {
            speed = max_speed;
        } else if angle.abs() >= max_angle {
            speed = min_speed;
        } else {
            let angle_ratio = (angle.abs() - threshold_angle) / (max_angle - threshold_angle);
            let distance_ratio = (distance - threshold_distance) / (1000.0 - threshold_distance);
            speed = max_speed - (angle_ratio.max(distance_ratio) * (max_speed - min_speed));
        }
    
        // convert the speed to an integer and return
        speed as i32
    }

    


    // Test case 1: distance = 2000, angle = 15
    let distance = 2000.0;
    let angle = 15.0;
    let expected_speed = 100;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 1 passed");
    } else {
        println!("Test case 1 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 2: distance = 900, angle = 25
    let distance = 900.0;
    let angle = 25.0;
    let expected_speed = 100;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 2 passed");
    } else {
        println!("Test case 2 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 3: distance = 800, angle = 35
    let distance = 800.0;
    let angle = 35.0;
    let expected_speed = 95;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 3 passed");
    } else {
        println!("Test case 3 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 4: distance = 700, angle = 45
    let distance = 700.0;
    let angle = 45.0;
    let expected_speed = 91;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 4 passed");
    } else {
        println!("Test case 4 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 5: distance = 600, angle = 55
    let distance = 600.0;
    let angle = 55.0;
    let expected_speed = 86;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 5 passed");
    } else {
        println!("Test case 5 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 6: distance = 500, angle = 65
    let distance = 500.0;
    let angle = 65.0;
    let expected_speed = 81;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 6 passed");
    } else {
        println!("Test case 6 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 7: distance = 400, angle = 75
    let distance = 400.0;
    let angle = 75.0;
    let expected_speed = 76;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 7 passed");
    } else {
        println!("Test case 7 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 8: distance = 300, angle = 85
    let distance = 300.0;
    let angle = 85.0;
    let expected_speed = 71;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 8 passed");
    } else {
        println!("Test case 8 failed. Expected: {}, Got: {}", expected_speed, result);
    }

    // Test case 9: distance = 200, angle = 95
    let distance = 200.0;
    let angle = 95.0;
    let expected_speed = 66;
    let result = ideal_speed(distance, angle);
    if result == expected_speed {
        println!("Test case 9 passed");
    } else {
        println!("Test case 9 failed. Expected: {}, Got: {}", expected_speed, result);
    }

}
