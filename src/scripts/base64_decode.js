/**
{
    "api": 1,
    "name": "Decode Base64",
    "description": "Decode Base64.",
    "author": "MGlolenstine",
    "icon": "html",
    "tags": "base64,decode"
}
**/

function main(state) {
    state.fullText = atob(state.fullText);
}