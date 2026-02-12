use unidecode::unidecode;
pub fn sanitize(input: &String) -> String {
	let s = unidecode(
		&input
			.replace(" ", "_")
			.replace("&", "and")
			.replace("#", "pound")
			.replace("*", "asterisk")
			.replace("1st", "first")
			.replace("2nd", "second")
			.replace("3rd", "third")
			.replace("(", "")
			.replace(")", "")
			.replace(":", "")
			.replace(".", "")
			.replace(".", "")
			.replace("'", "")
			.replace("’", "")
			.replace(",", "")
			.replace(",", "")
			.replace(",", "")
			.replace("-", "_")
			.replace("-", "_")
			.replace("“", "_")
			.replace("”", "_")
			.replace("!", "")
            .replace("+", "plus"),
	);
    if s.chars().next().map_or(false, |c| c.is_digit(10)) {
        format!("n{}", s)
    } else {
        s
    }
}
