extern crate octocrab;

use serde_json;

use octocrab::{Octocrab, Page, Result, models::{self, repos::RepoCommit}, params};
use octocrab::{tokio::net::windows::named_pipe::PipeEnd::Client, Error};
use octocrab::{Client, Error};
use std::error::Error;
use chrono::{Duration, Utc, NaiveDate};

pub fn init_octo(token: String) -> Result<Octocrab, octocrab::Error>
{
    (Octocrab::builder().personal_token(token).build())
}

pub async fn get_repo(token: String, owner: String, repo: String) -> Result<octocrab::models::Repository, ()> {
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let repo = match octo.repos(owner, repo).get().await {
        Ok(repo) => repo,
        Err(_) => return Err(()),
    };
    Ok(repo)
}

pub async fn get_issue(token: String, owner: String, repo: String) -> Result<Page<octocrab::models::issues::Issue>, octocrab::Error> {
    let route = format!("repos/{owner}/{repo}/issues/");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let mut page = octo.issues(owner, repo).list().state(params::State::All).send().await?;

    let mut i = 0;
    loop {
        for issue in &page.items {
            //println!("{}: {} -- {}", i, issue.title, issue.created_at);
            i += 1;
        }
        page = match octo
            .get_page::<models::issues::Issue>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }

    Ok(page)
}

pub async fn get_issues(token: String, owner: String, repo: String, t: i64) -> Result<Vec<Vec<octocrab::models::issues::Issue>>> {
    let route = format!("repos/{owner}/{repo}/issues/");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    
    let mut page = octo.issues(owner, repo).list().state(params::State::All).send().await?;
    let mut i = 0;
    let bin_size = chrono::Duration::days(t);
    let mut issues = Vec::new();
    let mut all_issues = Vec::new();

    loop {
        for issue in &page.items {

            let created_at = issue.created_at.naive_utc();
            let bin_start = created_at.date().and_hms_opt(0, 0, 0);
            let bin_end = bin_start.unwrap() + bin_size;
            if created_at >= bin_start.unwrap() && created_at < bin_end {
                issues.push(issue.clone());
            }
            if created_at >= bin_end {
                all_issues.push(issues.clone());
                issues = Vec::new();
            }
        }

        if !issues.is_empty() {
            all_issues.push(issues.clone());
        }
        
        page = match octo
            .get_page::<models::issues::Issue>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }

    Ok(all_issues)
}

pub async fn get_commits(token: String, owner: String, repo: String, t: f64) -> Result<Vec<Vec<RepoCommit>>, octocrab::Error> {
    let route = format!("repos/{owner}/{repo}/commits");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let mut page = octo.repos(owner, repo).list_commits().since(Utc::now() - Duration::days(t as i64)).send().await?;

    let mut i = 0;
    let mut bin_number = 0;
    let mut bins = vec![vec![]];

    loop {
        for commit in &page.items {
            //println!("{}: {} -- {}", i, commit.sha, commit.commit.message);
            bins[bin_number].push(commit.clone());
            i += 1;
        }
        match octo
            .get_page(&page.next)
            .await
        {
            Ok(Some(next_page)) => page = next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }
    Ok(bins)
}

pub async fn uses_workflows(token: String, owner: String, repo: String) -> Result<bool, octocrab::Error> {
    //
    let route = format!("repos/{owner}/{repo}/commits");
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let workflows = octo.workflows(owner, repo).list_all_runs().send().await?;

    let mut uses_workflows: bool = false;
    if !workflows.items.is_empty(){
        uses_workflows = true;
    }
    
    Ok((uses_workflows))
}

pub async fn get_bugs(token: String, owner: String, repo: String) -> Result<Vec<octocrab::models::issues::Issue>, octocrab::Error>{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let label = "bug".to_string();
    let labels = vec![label];
    let mut page = octo.issues(owner, repo).list().labels(&labels).send().await?;

    let mut bugs = Vec::new();
    loop {
        bugs.extend(page.items.into_iter());

        page = match octo
            .get_page::<models::issues::Issue>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }

    for bug in bugs.clone(){
        println!("{:#?}", bug.body_text);
    }

    Ok(bugs)
}

pub async fn get_all_commits(token: String, owner: String, repo: String) -> Result<Vec<RepoCommit>, octocrab::Error>{
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let mut page= octo.repos(owner, repo).list_commits().send().await?;

    let mut commits = Vec::new();
    loop {
        commits.extend(page.items.into_iter());

        page = match octo
            .get_page::<RepoCommit>(&page.next)
            .await
        {
            Ok(Some(next_page)) => next_page,
            Ok(None) => break,
            Err(_) => break,
        }
    }
    Ok((commits))
}

pub async fn get_duration_between_first_and_last_commit(token: String, owner: String, repo: String) -> Result<f64, octocrab::Error> {
    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    let first_commit_page = octo
        .repos(owner.clone(), repo.clone())
        .list_commits()
        .send()
        .await?;

    let most_recent_commit_page = octo
        .repos(owner, repo)
        .list_commits()
        .page(1 as u32)
        .per_page(1)
        .send()
        .await?;

    let first_commit = &first_commit_page.items[first_commit_page.items.len() - 1];
    let most_recent_commit = &most_recent_commit_page.items[0];

    let first_commit_sha = &first_commit.sha.get(0..7).unwrap();
    let most_recent_commit_sha = &most_recent_commit.sha.get(0..7).unwrap();

    let first_commit_date = NaiveDate::parse_from_str(
        &format!("{}-{}-{}", &first_commit_sha[0..4], &first_commit_sha[4..6], &first_commit_sha[6..7]),
        "%Y-%m-%d",
    )
    .unwrap();

    let most_recent_commit_date = NaiveDate::parse_from_str(
        &format!("{}-{}-{}", &most_recent_commit_sha[0..4], &most_recent_commit_sha[4..6], &most_recent_commit_sha[6..7]),
        "%Y-%m-%d",
    )
    .unwrap();

    let duration = most_recent_commit_date.signed_duration_since(first_commit_date);
    let elapsed_days = duration.num_days() as f64;
    
    Ok(elapsed_days)
}

// Example of using GraphQL
pub async fn get_num_commits(token: String, owner: &str, repo: &str) -> serde_json::Value
{
    let v = vec!["query {repository(owner: \"", owner, "\", name: \"", repo, "\") {object(expression: \"master\") {... on Commit {history {totalCount}}}}}"];

    let str: String = v.concat();

    println!("{}", str);

    let octo = Octocrab::builder().personal_token(token).build().unwrap();

    match octo.graphql(&str).await
    {
        Ok(json) => json,
        Err(_) => panic!("Error with query"),
    }
}

// This function returns a vector of tuples. Each tuple contains a contrubutor, the number of
// contributions they made, and the percentage of total contributions that they made. This
// vector saved as 'contributors' which is a vector of tuples. It is saved as the result.
pub async fn get_contributors_with_percentages(token: String, owner: String, repo: String) -> Result<Vec<(octocrab::models::repos::Contributor, i32, f32)>, OctocrabError> {
    let octo = Octocrab::builder().personal_token(token).build().unwrap();
    let contributors = octo.repos(owner, repo).list_contributors().send().await?;
    let mut contributor_list = vec![];

    for contributor in contributors.items {
        contributor_list.push((contributor, contributor.contributions));
    }

    let total_contributions: i32 = contributor_list.iter().map(|(_, contributions)| *contributions).sum();
    let mut result = vec![];

    for (contributor, contributions) in contributor_list {
        let percentage = (contributions as f32 / total_contributions as f32) * 100.0;
        result.push((contributor, contributions, percentage));
    }

    Ok(result)
}

// Our group determined that the presense of a README was the most important part of
// the ramp up score. A README allows others to get versed in a project and learn what
// its about. If there is a README the repository will recieve a score of 1. If not
// it will get a score of 0.
pub async fn has_readme(repo: &str, client: &Client) -> Result<i32, Error> {
    let file_list_url = format!("/repos/{}/git/trees/master?recursive=1", repo);
    let file_list_response = client
        .get(file_list_url)
        .send()
        .await?;

    let file_list_content = file_list_response.json::<serde_json::Value>().await?;
    let files = file_list_content["tree"].as_array().unwrap();

    for file in files {
        let path = file["path"].as_str().unwrap();
        if path == "README.md" {
            return Ok(1);
        }
    }

    Ok(0)
}

pub async fn check_multiple_readmes(repo: &str, client: &Client) -> Result<i32, Error> {
    let contents_url = format!("/repos/{}/contents/", repo);
    let contents_response = client.get(contents_url).send().await?;

    let contents = contents_response.json::<Vec<serde_json::Value>>().await?;

    let mut readme_count = 0;

    for content in contents {
        let name = content["name"].as_str().unwrap();
        if name.to_lowercase().starts_with("readme") {
            readme_count += 1;
        }
    }

    if readme_count > 1 {
        Ok(1)
    } else {
        Ok(0)
    }
}

pub async fn are_all_issues_closed(repo: &str, client: &Client) -> Result<i32, Error> {
    let issues_url = format!("/repos/{}/issues", repo);
    let issues_response = client.get(issues_url).send().await?;

    let issues = issues_response.json::<Vec<serde_json::Value>>().await?;

    for issue in issues {
        let state = issue["state"].as_str().unwrap();
        if state != "closed" {
            return Ok(0);
        }
    }

    Ok(1)
}

// has tests (1/3 weight)
pub async fn has_testing_suite(repo: &str, client: &Client) -> Result<i32, Error> {
    let contents_url = format!("/repos/{}/contents", repo);
    let contents_response = client.get(contents_url).send().await?;

    let contents = contents_response.json::<Vec<serde_json::Value>>().await?;

    for content in contents {
        let content_name = content["name"].as_str().unwrap();
        if content_name == "tests" {
            return Ok(1);
        }
    }

    Ok(0)
}

// check number of releases (1/3 weight)
pub async fn check_number_of_releases(repo: &str, client: &Client) -> Result<i32, Error> {
    let releases_url = format!("/repos/{}/releases", repo);
    let releases_response = client.get(releases_url).send().await?;

    let releases = releases_response.json::<Vec<serde_json::Value>>().await?;

    if releases.len() > 10 {
        Ok(1)
    } else {
        Ok(0)
    }
}