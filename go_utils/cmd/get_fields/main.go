package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

func main() {
	if len(os.Args) <= 1 {
		log.Fatal("no args given -- expected a goquery selector")
	}
	selector := os.Args[1]

	res, err := http.Get("https://developers.dailymotion.com/api")
	if err != nil {
		log.Fatal(err)
	}
	defer res.Body.Close()
	if res.StatusCode != 200 {
		log.Fatalf("status code error: %d %s", res.StatusCode, res.Status)
	}

	doc, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		log.Fatal(err)
	}

	selection := doc.Find(selector)
	if selection.Size() != 1 {
		log.Fatalf("expected 1 selection, selector found %d", selection.Size())
	}
	rows := selection.First().Parent().Find("dl").First().Find("dt")

	fields := *new([]field)
	rows.Each(func(_ int, s *goquery.Selection) {
		fields = append(fields, parse(s))
	})

	rust_code := build_rust_enum(fields, true)

	f, err := os.Create("out.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	f.WriteString(rust_code)
}

type field struct {
	Typ  string
	Name string
}

func parse(s *goquery.Selection) (f field) {
	f.Typ = to_rust_type(s.Find("code.type a").First().Text())
	f.Name = s.Find("code.callname span").First().Text()
	return
}

func to_rust_type(typ string) string {
	switch strings.ToLower(typ) {
	// string
	case "string":
		fallthrough
	case "url":
		fallthrough
	case "channel":
		fallthrough
	case "user":
		fallthrough
	case "video":
		fallthrough
	case "identifier":
		return "String"

  // Datetime
	case "date":
    return "DateTime<Utc>"

	// int
	case "number":
		return "i32"

	// bool
	case "boolean":
		return "bool"

	// arr/vec
	case "array":
		return "Vec<String>"

	// hashmap/dict
	case "dict":
		return "HashMap<String, String>"

	// invalid
	default:
		log.Fatalf("invalid type %s", typ)
		return ""
	}
}

func build_rust_enum(fields []field, pub bool) string {
	// this section is for the response type
	output := "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n"
	if pub {
		output += "pub "
	}
	output += "struct FooResponse {\n"
	for _, field := range fields {
    output += with_attrib(field)
		output += "    "
		if pub {
			output += "pub "
		}
		output += fmt.Sprintf("%s: Option<%s>,\n", field.Name, field.Typ)
	}
	output += "}\n\n"

	// and this one is for the enum field
	output += "#[allow(clippy::module_name_repetitions, non_camel_case_types)]\n"
	output += "#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]\n"
	if pub {
		output += "pub "
	}
	output += "enum FooField {\n"
	for _, field := range fields {
		output += fmt.Sprintf("    %s,\n", field.Name)
	}
	output += "}"

	return output
}

func with_attrib(f field) string {
    if f.Typ == "DateTime<Utc>" {
        return "    #[serde(default, deserialize_with = \"from_tsopt\")]\n"
    }

    return ""
}

