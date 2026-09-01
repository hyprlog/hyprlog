A few goals I want to keep in mind when it comes to the code:
- Every part of a function body should be at roughly the same level of abstraction
- Each part of the code should be fully understandable on its own. Nothing should require massive working memory remembering obscure details from other files or distant parts of the same file.
- All assumptions should be built into the code. Incorrect state should be impossible or should throw clear errors.
- Any changes made by AI need to be as minimal and tightly scoped as possible. Agents do not have permission to run Roughshod over the code base. Understanding and familiarity are important. Don't change things unless you are asked to.
