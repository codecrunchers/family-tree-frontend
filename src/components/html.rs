//!Single home for most of the HTML
use crate::components::{BioPanel, GraphPanel, SearchButton};
use rusted_cypher::cypher::result::{CNode, CypherGraphResult};

macro_rules! remove_quotes {
    ($input:expr) => {
        $input.replace("\"", "")
    };
}

use wasm_bindgen::prelude::*;
use yew::prelude::*;

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
                        <SearchButton on_search=search_handler.clone() />
                     </div>
                   </div>

                   <div class="card mb-4">
                     <div class="card-body">
                           <BioPanel family=family.clone() on_search=search_handler.clone() />
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
 * it accepts two handle, the select handler sets the Person to be searched for to the value of the
 * image alt tag, the second just invokes search
 **/
pub fn bio_panel_bio(
    n: &CNode,
    bio_select_handle: yew::Callback<MouseEvent>,
    bio_search_handle: yew::Callback<MouseEvent>,
) -> yew::Html {
    let name: String = n.properties.get("fullName").unwrap().to_string();
    let name = remove_quotes!(name);
    yew::html! {
        <div class="col">
            <div class="card">
            {
                if n.properties.get("gender").unwrap() == "female"  {
                    yew::html!{
                        <img src="imgs/unknown_female.png" class="card-img-top" alt={name.clone()} onmousedown={bio_select_handle} onclick={bio_search_handle}/>
                    }
                }else{
                    yew::html!{
                        <img src="imgs/unknown_male.png" class="card-img-top" alt={name.clone()} onmousedown={bio_select_handle} onclick={bio_search_handle}/>
                    }
                }
            }

            <h5 class="card-title">{name}</h5>
           { for n.properties.iter().map(|hme| {
                     html!{
                         <div class="card-body">
                             <h7 class="card-title">{hme.0}</h7>
                             <p class="card-text">{hme.1}</p>
                          </div>
                     }
                 })
           }

        </div>
            <p class="card-text">
            <small class="text-muted">{"Last updated 3 mins ago"}</small>
            </p>
        </div>
    }
}
