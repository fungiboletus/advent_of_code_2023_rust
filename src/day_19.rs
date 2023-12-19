/*
    Part 1: Look relatively straightforward.
        - Parse the input data
        - Get a hashmap of workflows.
        - Hope for the best.
        - Part 2 is going to be a nightmare.
*/

use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, Clone)]
struct RuleCondition {
    category: Category,
    value: u64,
    greter_lower: GreaterLower,
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
                greter_lower,
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

            let condition_result = match condition.greter_lower {
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
                let destination = rule.send_destination.as_ref().unwrap();
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
        .map(|part_rating| part_rating.x + part_rating.m + part_rating.a + part_rating.s)
        .sum::<u64>() as i64
}

pub fn day_19_part_2(data: &str) -> i64 {
    42
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
    fn test_day_19_part_2() {
        assert_eq!(day_19_part_2(EXAMPLE), 167409079868000);
    }
}
