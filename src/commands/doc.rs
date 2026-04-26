use crate::{
    logger::{Severity, log},
    meta::{display_full_license, project_information::PROJECT},
    output,
    utils::{
        colors::COLORS,
        fs::{delete_file, download_file_to_location_via_curl, use_pager_on_file},
        io::error,
        net::not_connected_to_internet,
    },
};

fn list_available_options() {
    let (magenta, blue, cyan) = (COLORS.magenta, COLORS.blue, COLORS.cyan);

    output!(
        "Available options:\n\
        {magenta}----Main Repository Information----\n\
        {blue}main              {cyan}The main README file.\n\
        {blue}changelog         {cyan}The complete version history of cmdcreate.\n\
        {blue}license           {cyan}The license for cmdcreate.\n\
        {blue}security          {cyan}Security information.\n\
        {blue}contributing      {cyan}Information about contributing.\n\
        {blue}code_of_conduct   {cyan}View information about the code of conduct.\n\
        {magenta}----Main Documentation----\n\
        {blue}intro             {cyan}Intro to the documentation.\n\
        {blue}about             {cyan}About cmdcreate and its purpose.\n\
        {blue}commands          {cyan}About the current commands in cmdcreate.\n\
        {blue}configurations    {cyan}Information of configuring cmdcreate.\n\
        {blue}developing        {cyan}Information on developing cmdcreate.\n\
        {blue}structure         {cyan}The file structure of cmdcreate.\n\
        {blue}updates           {cyan}Information on updates.\n\
        {magenta}----Other Information for Development----\n\
        {blue}testing           {cyan}Information about testing the features of cmdcreate.\n\
        {blue}packaging         {cyan}Information about packaging cmdcreate.",
        true
    );
}

pub fn doc(info_to_retrieve: &str) {
    if not_connected_to_internet() {
        error(
            "You need internet to continue!",
            "Please connect to the internet!",
        )
    }

    if info_to_retrieve == "list" {
        list_available_options();
        return;
    }

    // The file path starts from the root of the repository.
    // Files like README.md have just the file name, but
    // the main documentation would be assigned to the path
    // docs/documentation.md.
    //
    // Note:
    // "license" and "changelog" are exempt because they
    // already have their own dedicated functions.
    let doc_file_path: &str = match info_to_retrieve {
        "main" => "README.md",
        "changelog" => "changes.md",
        "license" => "none",
        "security" | "contributing" | "code_of_conduct" => {
            &format!("{}.md", info_to_retrieve.to_uppercase())
        }
        "intro" => "docs/README.md",
        "about" | "commands" | "configurations" | "developing" | "structure" | "updates" => {
            &format!("docs/{info_to_retrieve}.md")
        }
        "testing" => "testing/README.md",
        "packaging" => "package/README.md",

        _ => error("Invalid option:", info_to_retrieve),
    };

    log(
        &format!(
            "commands/doc::doc(): Getting information for: \
            {info_to_retrieve} (file \"{doc_file_path}\")"
        ),
        Severity::Normal,
    );

    // Manual override, there is already a dedicated function
    // to retrieve the license. Extra steps are needed so the
    // function is kept.
    if info_to_retrieve == "license" {
        display_full_license();
    }

    view_documentation_file(doc_file_path);
}

pub fn view_documentation_file(file_path_name: &str) {
    let repo_path = PROJECT.repository_raw;
    let doc_file_download_path = &format!("{repo_path}/{file_path_name}");

    let temp_doc_file_path = "/tmp/cmdcreate_doc_temp.md";
    download_file_to_location_via_curl(temp_doc_file_path, doc_file_download_path);

    use_pager_on_file(temp_doc_file_path);

    delete_file(temp_doc_file_path).expect("Failed to delete doc file");
}
