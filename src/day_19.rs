/*
    Part 1: Look relatively straightforward.
        - Parse the input data
        - Get a hashmap of workflows.
        - Hope for the best.
        - Part 2 is going to be a nightmare.
*/

use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, char, line_ending, u64},
    combinator::{map, value},
    multi::{count, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RuleType {
    Send,
    Reject,
    Accept,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GreaterLower {
    Greater,
    Lower,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleCondition {
    category: Category,
    value: u64,
    greater_lower: GreaterLower,
}

impl RuleCondition {
    fn invert(&self) -> RuleCondition {
        let greater_lower = match self.greater_lower {
            GreaterLower::Greater => GreaterLower::Lower,
            GreaterLower::Lower => GreaterLower::Greater,
        };

        // take into account that is not greater or equal…
        let value = match self.greater_lower {
            GreaterLower::Greater => self.value + 1,
            GreaterLower::Lower => self.value - 1,
        };

        RuleCondition {
            category: self.category,
            value,
            greater_lower,
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    rule_type: RuleType,
    condition: Option<RuleCondition>,
    send_destination: Option<String>,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct PartRating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl PartRating {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_rule_type_and_destination(data: &str) -> IResult<&str, (RuleType, Option<String>)> {
    alt((
        value((RuleType::Reject, None), tag("R")),
        value((RuleType::Accept, None), tag("A")),
        map(alphanumeric1, |destination: &str| {
            (RuleType::Send, Some(destination.to_string()))
        }),
    ))(data)
}

fn parse_conditional_rule(data: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            alt((
                value(Category::X, tag("x")),
                value(Category::M, tag("m")),
                value(Category::A, tag("a")),
                value(Category::S, tag("s")),
            )),
            alt((
                value(GreaterLower::Greater, tag(">")),
                value(GreaterLower::Lower, tag("<")),
            )),
            u64,
            tag(":"),
            parse_rule_type_and_destination,
        )),
        |(category, greter_lower, value, _, destination)| Rule {
            rule_type: destination.0,
            condition: Some(RuleCondition {
                category,
                value,
                greater_lower: greter_lower,
            }),
            send_destination: destination.1,
        },
    )(data)
}

fn parse_rules(data: &str) -> IResult<&str, Vec<Rule>> {
    separated_list0(
        char(','),
        alt((
            parse_conditional_rule,
            map(
                parse_rule_type_and_destination,
                |(rule_type, destination)| Rule {
                    rule_type,
                    condition: None,
                    send_destination: destination,
                },
            ),
        )),
    )(data)
}

fn parse_workflow(data: &str) -> IResult<&str, Workflow> {
    map(
        tuple((take_until("{"), delimited(tag("{"), parse_rules, tag("}")))),
        |(name, rules)| Workflow {
            name: name.to_string(),
            rules,
        },
    )(data)
}

fn parse_workflows(data: &str) -> IResult<&str, Vec<Workflow>> {
    separated_list0(line_ending, parse_workflow)(data)
}

fn parse_part_rating(data: &str) -> IResult<&str, PartRating> {
    map(
        delimited(
            tag("{"),
            tuple((
                preceded(tag("x="), u64),
                preceded(tag(",m="), u64),
                preceded(tag(",a="), u64),
                preceded(tag(",s="), u64),
            )),
            tag("}"),
        ),
        |(x, m, a, s)| PartRating { x, m, a, s },
    )(data)
}

fn parse_part_ratings(data: &str) -> IResult<&str, Vec<PartRating>> {
    separated_list0(line_ending, parse_part_rating)(data)
}

fn parse_input_data(data: &str) -> IResult<&str, (Vec<Workflow>, Vec<PartRating>)> {
    map(
        tuple((parse_workflows, count(line_ending, 2), parse_part_ratings)),
        |(workflows, _, part_ratings)| (workflows, part_ratings),
    )(data)
}

fn process_part_rating(part_rating: &PartRating, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current_workflow = workflows.get("in").expect("Failed to find in workflow");

    // CurrentRules as a VecDeque
    let mut current_rules: VecDeque<Rule> = current_workflow.rules.iter().cloned().collect();

    while let Some(rule) = current_rules.pop_front() {
        if let Some(condition) = rule.condition {
            let value = match condition.category {
                Category::X => part_rating.x,
                Category::M => part_rating.m,
                Category::A => part_rating.a,
                Category::S => part_rating.s,
            };

            let condition_value = condition.value;

            let condition_result = match condition.greater_lower {
                GreaterLower::Greater => value > condition_value,
                GreaterLower::Lower => value < condition_value,
            };

            if !condition_result {
                continue;
            }
        }

        match rule.rule_type {
            RuleType::Accept => {
                return true;
            }
            RuleType::Reject => {
                return false;
            }
            RuleType::Send => {
                let destination = rule
                    .send_destination
                    .as_ref()
                    .expect("Send rule without destination");
                current_workflow = workflows.get(destination).expect("Failed to find workflow");
                current_rules = current_workflow.rules.iter().cloned().collect();
            }
        }
    }

    panic!("Failed to finish workflow, weird");
}

pub fn day_19_part_1(data: &str) -> i64 {
    let (_, workflows_and_part_ratings) =
        parse_input_data(data).expect("Failed to parse input data");
    let (workflows, part_ratings) = workflows_and_part_ratings;

    let hashmap_workflows: std::collections::HashMap<String, Workflow> = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow.clone()))
        .collect();

    // for each part rating, find the matching workflow
    part_ratings
        .iter()
        .filter(|part_rating| process_part_rating(&part_rating, &hashmap_workflows))
        .map(|part_rating| part_rating.sum())
        .sum::<u64>() as i64
}

/**
 *
 * Part 2: We could consider the workflows as a graph, and look for all the paths
 * while keeping track of the ranges.
 */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchRange {
    min_x: u64,
    max_x: u64,

    min_m: u64,
    max_m: u64,

    min_a: u64,
    max_a: u64,

    min_s: u64,
    max_s: u64,
}

impl Default for SearchRange {
    fn default() -> Self {
        SearchRange {
            min_x: 1,
            max_x: 4000,

            min_m: 1,
            max_m: 4000,

            min_a: 1,
            max_a: 4000,

            min_s: 1,
            max_s: 4000,
        }
    }
}

impl SearchRange {
    fn is_valid(&self) -> bool {
        self.min_x <= self.max_x
            && self.min_m <= self.max_m
            && self.min_a <= self.max_a
            && self.min_s <= self.max_s
    }

    fn apply_rule_condition(&self, rule_condition: &RuleCondition) -> Option<SearchRange> {
        let mut new_search_range = self.clone();

        let RuleCondition {
            category,
            value,
            greater_lower,
        } = rule_condition;

        match category {
            Category::X => match greater_lower {
                GreaterLower::Greater => {
                    new_search_range.min_x = value + 1;
                }
                GreaterLower::Lower => {
                    new_search_range.max_x = value - 1;
                }
            },
            Category::M => match greater_lower {
                GreaterLower::Greater => {
                    new_search_range.min_m = value + 1;
                }
                GreaterLower::Lower => {
                    new_search_range.max_m = value - 1;
                }
            },
            Category::A => match greater_lower {
                GreaterLower::Greater => {
                    new_search_range.min_a = value + 1;
                }
                GreaterLower::Lower => {
                    new_search_range.max_a = value - 1;
                }
            },
            Category::S => match greater_lower {
                GreaterLower::Greater => {
                    new_search_range.min_s = value + 1;
                }
                GreaterLower::Lower => {
                    new_search_range.max_s = value - 1;
                }
            },
        }

        match new_search_range.is_valid() {
            true => Some(new_search_range),
            false => None,
        }
    }

    #[allow(dead_code)]
    fn contains_part_rating(&self, part_rating: &PartRating) -> bool {
        self.min_x <= part_rating.x
            && part_rating.x <= self.max_x
            && self.min_m <= part_rating.m
            && part_rating.m <= self.max_m
            && self.min_a <= part_rating.a
            && part_rating.a <= self.max_a
            && self.min_s <= part_rating.s
            && part_rating.s <= self.max_s
    }

    #[allow(dead_code)]
    fn contains_search_range(&self, search_range: &SearchRange) -> bool {
        self.min_x <= search_range.min_x
            && search_range.max_x <= self.max_x
            && self.min_m <= search_range.min_m
            && search_range.max_m <= self.max_m
            && self.min_a <= search_range.min_a
            && search_range.max_a <= self.max_a
            && self.min_s <= search_range.min_s
            && search_range.max_s <= self.max_s
    }

    fn nb_combinations(&self) -> u64 {
        (self.max_x - self.min_x + 1)
            * (self.max_m - self.min_m + 1)
            * (self.max_a - self.min_a + 1)
            * (self.max_s - self.min_s + 1)
    }
}

fn compute_valid_ranges(workflows: &[Workflow]) -> Vec<SearchRange> {
    let hashmap_workflows: std::collections::HashMap<String, Workflow> = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow.clone()))
        .collect();

    // do a DFS on the workflows, and count the number of solutions
    let mut stack: Vec<(String, SearchRange)> = Vec::new();

    // To go faster, we skip the workflows that we already visited
    // with the exact same search range
    let mut visited: HashSet<(String, SearchRange)> = HashSet::new();

    // start with the "in" workflow
    stack.push(("in".to_string(), SearchRange::default()));

    let mut valid_ranges: Vec<SearchRange> = Vec::new();

    while let Some(stack_element) = stack.pop() {
        if visited.contains(&stack_element) {
            //println!("Already visited: {:?}", stack_element);
            continue;
        }

        visited.insert(stack_element.clone());

        let (workflow_name, search_range) = stack_element;

        let workflow = hashmap_workflows
            .get(&workflow_name)
            .expect("Failed to find workflow");

        let mut work_search_range = search_range.clone();

        for rule in workflow.rules.iter() {
            let Rule {
                rule_type,
                condition,
                send_destination,
            } = rule;
            //println!("{:?}", rule);
            //println!("{:?}", work_search_range);

            let mut current_search_range = work_search_range.clone();

            if !current_search_range.is_valid() {
                //println!("Invalid search range: {:?}", current_search_range);
                continue;
            }

            if let Some(condition) = &condition {
                if let Some(new_search_range) = work_search_range.apply_rule_condition(condition) {
                    current_search_range = new_search_range;
                }
                let inverted_condition = condition.invert();
                if let Some(new_search_range) =
                    work_search_range.apply_rule_condition(&inverted_condition)
                {
                    work_search_range = new_search_range.clone();
                }
            }

            match rule_type {
                RuleType::Accept => {
                    if search_range.is_valid() {
                        //println!("Found a solution: {:?}", current_search_range);
                        valid_ranges.push(current_search_range);
                    } else {
                        panic!("WTF");
                    }
                }
                RuleType::Reject => {
                    continue;
                }
                RuleType::Send => {
                    let destination = send_destination
                        .as_ref()
                        .expect("Send rule without destination");
                    let new_stack_element: (String, SearchRange);
                    new_stack_element = (destination.clone(), current_search_range.clone());
                    if !visited.contains(&new_stack_element) {
                        stack.push(new_stack_element);
                    }
                }
            }
        }
    }

    return valid_ranges;
}

pub fn day_19_part_2(data: &str) -> i64 {
    let (_, workflows) = parse_workflows(data).expect("Failed to parse input data");
    let valid_ranges = compute_valid_ranges(&workflows);

    // it looks like there is no overlap between the valid ranges in the input data
    valid_ranges
        .iter()
        .map(|search_range| search_range.nb_combinations())
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_day_19_part_1() {
        assert_eq!(day_19_part_1(EXAMPLE), 19114);
    }

    #[test]
    fn test_day_19_part_1_2() {
        // alternative way to solve part 1 by using the part 2 logic
        let (_, workflows_and_part_ratings) =
            parse_input_data(EXAMPLE).expect("Failed to parse input data");
        let (workflows, part_ratings) = workflows_and_part_ratings;

        let valid_ranges = compute_valid_ranges(&workflows);

        let sum = part_ratings
            .iter()
            .filter(|part_rating| {
                valid_ranges
                    .iter()
                    .any(|range| range.contains_part_rating(part_rating))
            })
            .map(|part_rating| part_rating.sum())
            .sum::<u64>();
        assert_eq!(sum, 19114);
    }

    #[test]
    fn test_day_19_part_2() {
        assert_eq!(day_19_part_2(EXAMPLE), 167409079868000);
    }

    #[test]
    fn test_rule_condition() {
        let rule_condition = RuleCondition {
            category: Category::X,
            value: 100,
            greater_lower: GreaterLower::Greater,
        };
        // 102 true
        // 101 true
        // 100 false
        // 99 false

        assert_eq!(
            rule_condition.invert(),
            RuleCondition {
                category: Category::X,
                value: 101,
                greater_lower: GreaterLower::Lower,
            }
        );

        let rule_condition = RuleCondition {
            category: Category::X,
            value: 999,
            greater_lower: GreaterLower::Lower,
        };
        // 1000 false
        // 999 false
        // 998 true
        // 997 true

        assert_eq!(
            rule_condition.invert(),
            RuleCondition {
                category: Category::X,
                value: 998,
                greater_lower: GreaterLower::Greater,
            }
        );
    }

    #[test]
    fn day_19_no_ranges_overlap() {
        let (_, workflows) = parse_workflows(EXAMPLE).expect("Failed to parse input data");
        let valid_ranges = compute_valid_ranges(&workflows);
        // check if a range
        for i in 0..valid_ranges.len() {
            for j in i + 1..valid_ranges.len() {
                if valid_ranges[i].contains_search_range(&valid_ranges[j]) {
                    panic!(
                        "Found overlapping ranges: {:?} and {:?}",
                        valid_ranges[i], valid_ranges[j]
                    );
                }
            }
        }
    }
}
