/**
{
    "api": 1,
    "name": "Minimise code",
    "description": "Removes all spaces in the code to make it smaller",
    "author": "MGlolenstine",
    "icon": "html",
    "tags": "minimise,code,css,js"
}
**/

function main(state) {
    let splot = state.fullText.match(/[\"].+?[\"]|[^\s]+/g);
    state.fullText = splot.join("");
}