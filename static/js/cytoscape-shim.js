/**
 *  It's easier to grab DOM object here and add to WASM obj
 *  than figure out how to serialise the websys Elements
 */
function cytoscape_shim(cy_spec) {
    var element = document.getElementById("cy");
    cy_spec.container = element;
    console.log("Calling Cytoscape from Shim");
    var cy = cytoscape(cy_spec);

    cy.on('tap', 'node', function(){
        callback_test("123");

    });
}

