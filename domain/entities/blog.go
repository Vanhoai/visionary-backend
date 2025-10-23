package entities

type Blog struct {
	Base
	AuthorId    string `json:"author_id"`
	CategoryId  string `json:"category_id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Content     string `json:"content"`
	Stars       uint   `json:"stars"`
	Viewers     uint   `json:"viewers"`
}
