/**
	{
		"api":1,
		"name":"Base64 Encode",
		"description":"Encodes your text to Base64",
		"author":"See Source",
		"icon":"metamorphose",
		"tags":"base64,btoa,encode"
	}
**/

function main(input) {
    input.text = btoa(input.text);
}