use regex::Regex;
use std::{collections::HashMap, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let shift_data = convert_input(input)?;

    // Get guard with most time asleep
    let mut target_id = 0;
    let mut max_sleep = 0;

    for (guard_id, shifts) in shift_data.iter() {
        let mut local_max = 0;

        for shift in shifts {
            local_max += shift.iter().filter(|&x| *x).count();
        }

        if local_max > max_sleep {
            target_id = *guard_id;
            max_sleep = local_max;
        }
    }

    // Find most common minute for that guard to be asleep
    let mut times_asleep_at_min: HashMap<u32, u32> = HashMap::new();

    // Loop over shifts worked by most sleepy guard
    for shift in shift_data
        .get(&target_id)
        .expect("Failed to get shifts for sleepy guard")
    {
        // For each minute in their shift, check if they were asleep.
        // If so, increase count for that minute (i.e. `idx`)
        for (minute, was_asleep) in shift.iter().enumerate() {
            if *was_asleep {
                times_asleep_at_min
                    .entry(minute as u32)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    // Find most-asleep-minute in `times_asleep_at_min`
    let min_most_asleep: &u32 = times_asleep_at_min
        .iter()
        .max_by_key(|k| k.1)
        .expect("Failed to find single answer")
        .0;

    let answer = target_id * min_most_asleep;

    println!("Part 1: {}", answer);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let shift_data = convert_input(input)?;
    let sleep_freq_data = get_sleep_frequencies(&shift_data)?;

    let mut target_id: u32 = 0;
    let mut target_min: u32 = 0;
    let mut max: u32 = 0;

    for (id, freqs) in sleep_freq_data.iter() {
        let curr_max = freqs
            .iter()
            .fold(0, |max, &val| if val > max { val } else { max });
        let minute = freqs
            .iter()
            .position(|&count| count == curr_max)
            .expect("Failed to find position.");

        if curr_max > max {
            target_id = *id;
            target_min = minute as u32;
            max = curr_max;
        }
    }

    let answer = target_id * target_min;
    println!("Part 2: {}", answer);

    Ok(())
}

/*  Sleep schedules of guards stored in a hashmap:
K: Guard ID (String)
V: List of shifts that the guard worked. Each shift is a vec of
   bools representing whether the guard was asleep at that minute.

Example:
ID  SCHEDULE (# = true | . = false)
000000000011111111112222222222333333333344444444445555555555
012345678901234567890123456789012345678901234567890123456789
09  .............................................##########.....
10  .....####################.....#########################.....
........................#####...............................
99  ........................................##########..........
....................................##########..............

Guard #10 was asleep from 00:05 until 00:25, and 00:30 until
00:55 on shift #1, and asleep from 00:24 until 00:29 on shift #2.*/
fn convert_input(input: &str) -> Result<HashMap<u32, Vec<Vec<bool>>>> {
    // Sort events by timestamp
    let mut events: Vec<&str> = input.lines().collect();
    events.sort();

    fs::write("sorted_input.txt", &events.join("\n"))?;

    // Process events
    static NEW_GUARD_PATTERN: &str = r"Guard\s#(?P<id>\d+)\sbegins\sshift";
    let rgx_new_guard: Regex = Regex::new(NEW_GUARD_PATTERN)?;

    let mut shift_data: HashMap<u32, Vec<Vec<bool>>> = HashMap::new();
    let mut curr_guard_id = 0;
    let mut curr_schedule: Option<Vec<bool>> = None;

    for (idx, event) in events.iter().enumerate() {
        match rgx_new_guard.captures(event) {
            Some(capture) => {
                //New shift begins

                // If `curr_schedule` set, record last guard's shift before resetting
                if let Some(s) = curr_schedule {
                    shift_data
                        .get_mut(&curr_guard_id)
                        .expect("Failed to get schedule")
                        .push(s);
                }

                // Reset current schedule and ID for new guard
                curr_guard_id = capture[1].parse::<u32>().expect("Failed to parse guard id");
                curr_schedule = None;

                // Ensure entry in `shifts_worked`
                shift_data
                    .entry(curr_guard_id)
                    .or_insert(Vec::new());
            }
            None => {
                if event.contains("falls asleep") {
                    // Record Sleep in `curr_schedule`

                    // Grab existing schedule, or initialize
                    let mut schedule = if let Some(sch) = curr_schedule {
                        sch
                    } else {
                        vec![false; 60]
                    };

                    // Record time asleep
                    static FIND_MINUTES: &str = r"\d{2}:(?P<min>\d{2})";
                    let rgx_minutes: Regex = Regex::new(FIND_MINUTES).expect("Failed to create re");

                    let sleeps_at: usize = rgx_minutes
                        .captures(event)
                        .expect("Failed to read starting timestamp")[1]
                        .parse::<usize>()
                        .expect("Failed to parse starting minute");

                    let wakes_at: usize = rgx_minutes
                        .captures(events[idx + 1])
                        .expect("Failed to read ending timestamp")[1]
                        .parse::<usize>()
                        .expect("Failed to parse ending minute");

                    let minutes_asleep = wakes_at - sleeps_at;

                    for min in schedule.iter_mut().skip(sleeps_at).take(minutes_asleep) {
                        *min = true;
                    }

                    curr_schedule = Some(schedule);
                }
            }
        }
    }

    // Save final shift
    if let Some(s) = curr_schedule {
        shift_data
            .get_mut(&curr_guard_id)
            .expect("Failed to get schedule")
            .push(s);
    }

    Ok(shift_data)
}

fn get_sleep_frequencies(
    shift_data: &HashMap<u32, Vec<Vec<bool>>>,
) -> Result<HashMap<u32, Vec<u32>>> {
    let mut shift_freqs: HashMap<u32, Vec<u32>> = HashMap::new();

    for (id, shift_list) in shift_data.iter() {
        let mut freqs = vec![0u32; 60];

        for shift in shift_list {
            for (minute, was_asleep) in shift.iter().enumerate() {
                if *was_asleep {
                    freqs[minute] += 1;
                }
            }
        }

        shift_freqs.insert(*id, freqs);
    }

    Ok(shift_freqs)
}

#[allow(dead_code)] // Used for debugging
fn output_shift_data(map: &HashMap<u32, Vec<Vec<bool>>>) -> Result<()> {
    let mut shift_data_printable: Vec<String> = vec![
        format!("        000000000011111111112222222222333333333344444444445555555555"),
        format!("ID      012345678901234567890123456789012345678901234567890123456789"),
        format!("--------------------------------------------------------------------"),
    ];

    for (id, all_shifts) in map.iter() {
        shift_data_printable.push(id.to_string());
        let spacer = " ".repeat(8);

        for shift in all_shifts {
            let printable_shift = shift
                .iter()
                .map(|&was_asleep| match was_asleep {
                    true => "#",
                    false => ".",
                })
                .collect::<Vec<&str>>()
                .join("");

            shift_data_printable.push(format!("{}{}", &spacer, printable_shift));
        }
    }

    fs::write("shifts_worked.txt", shift_data_printable.join("\n"))?;

    Ok(())
}
