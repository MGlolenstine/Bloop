/**
{
    "api": 1,
    "name": "Encode Base64",
    "description": "Encode Base64.",
    "author": "MGlolenstine",
    "icon": "html",
    "tags": "base64,encode"
}
**/

function main(state) {
    state.fullText = btoa(state.fullText);
}