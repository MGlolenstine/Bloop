/**
	{
		"api":1,
		"name":"Base64 Decode",
		"description":"Decodes your text from Base64",
		"author":"See Source",
		"icon":"metamorphose",
		"tags":"base64,atob,decode"
	}
**/

function main(input) {
    input.text = atob(input.text);
}