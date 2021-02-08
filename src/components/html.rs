//!Single home for most of the HTML
use crate::components::{BioPanel, GraphPanel, SearchButton};
use rusted_cypher::cypher::result::{CNode, CypherGraphResult};

//Layout for Cytoscape
pub const LAYOUT: &'static str = r##"{
               "name":"cola",
               "convergenceThreshold":100,
               "animate":false
            }"##;

//Style for Cytoscape
pub const STYLE: &'static str = r##"[
            {
             "selector": "node",
              "style": {
               "label": "data(name)"
               }
              },
              {
                "selector": "edge",
                "css": {
                   "line-color": "#f92411"
                }
               }
              ]"##;

//Spinner for Homepage
pub fn home_loading() -> yew::Html {
    yew::html! {  <div class="loading_spinner_container">
                        <div class="loading_spinner"></div>
                        <div class="loading_spinner_text">{"Loading ..."}</div>
                    </div>
    }
}

/**
 * this is the main view for the bio panel, it takes a list of
 * html 'panels' as input
 */
pub fn home_view(
    family: CypherGraphResult,
    search_handler: yew::Callback<std::string::String>,
) -> yew::Html {
    yew::html! {
    <>
        <script src="https://code.jquery.com/jquery-3.2.1.slim.min.js" integrity="sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN" crossorigin="anonymous"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.12.9/umd/popper.min.js" integrity="sha384-ApNbgh9B+Y1QKtv3Rn7W3mgPxhU9K/ScQsAP7hUibX39j7fakFPskvXusvfa0b4Q" crossorigin="anonymous"></script>
    <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/js/bootstrap.min.js" integrity="sha384-JZR6Spejh4U02d8jOt6vLEHfe/JQGiRRSQQxSfFWpi1MquVdAyjUar5+76PVCmYl" crossorigin="anonymous"></script>

                                     <div class="container">
                                   <div class="row">
                                     <div class="col-md-8">

                                     <div class="card mb-4">
                                         <div class="card-body">
                                            <SearchButton on_search=search_handler />
                                         </div>
                                       </div>

                                       <div class="card mb-4">
                                         <div class="card-body">
                                               <BioPanel family=family.clone()/>
                                         </div>
                                       </div>

                                       <div class="card mb-4">
                                         <div class="card-body">
                                            <GraphPanel family=family.clone()/>
                                         </div>
                                       </div>

                                    </div>
                                 </div>
                               </div>
                               </>
        }
}

/**
 * this is the main view for the bio panel, it takes a list of
 * html 'panels' as input
 */
pub fn bio_panel_view(family: Vec<yew::Html>) -> yew::Html {
    let prev = "Previous";
    let next = "Next";
    yew::html! {
    <div class="container-fluid">
        <div class="scrolling-wrapper row flex-row flex-nowrap mt-4 pb-4 pt-2">
        {family}
        </div>
    </div>
    }
}

/**
 * this renders the Bio, accepts a CNode
 **/
pub fn bio_panel_bio(n: &CNode) -> yew::Html {
    yew::html! {
        <div class="col">
            <div class="card">
            {
                if n.properties.get("gender").unwrap() == "female"  {
                    yew::html!{
                        <img src="imgs/unknown_female.png" class="card-img-top" alt="{person}" />
                    }
                }else{
                    yew::html!{
                        <img src="imgs/unknown_male.png" class="card-img-top" alt="{person}" />
                    }
                }
            }
          <div class="card-body">
            <h5 class="card-title">{n.properties.get("fullName").unwrap()}</h5>
            <p class="card-text">{"
                                This is a wider card with supporting text below as a natural lead-in to
                                additional content. This content is a little bit longer."}
        </p>
        <p class="card-text">
          <small class="text-muted">{"Last updated 3 mins ago"}</small>
        </p>
      </div>
    </div>
    </div>
    }
}
