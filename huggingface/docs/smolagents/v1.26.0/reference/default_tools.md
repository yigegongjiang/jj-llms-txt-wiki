# Built-in Tools

Ready-to-use tool implementations provided by the `smolagents` library.

These built-in tools are concrete implementations of the [Tool](/docs/smolagents/v1.26.0/en/reference/tools#smolagents.Tool) base class, each designed for specific tasks such as web searching, Python code execution, webpage retrieval, and user interaction.
You can use these tools directly in your agents without having to implement the underlying functionality yourself.
Each tool handles a particular capability and follows a consistent interface, making it easy to compose them into powerful agent workflows.

The built-in tools can be categorized by their primary functions:
- **Information Retrieval**: Search and retrieve information from the web and specific knowledge sources.
  - [ApiWebSearchTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.ApiWebSearchTool)
  - [DuckDuckGoSearchTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.DuckDuckGoSearchTool)
  - [GoogleSearchTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.GoogleSearchTool)
  - [WebSearchTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.WebSearchTool)
  - [WikipediaSearchTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.WikipediaSearchTool)
- **Web Interaction**: Fetch and process content from specific web pages.
  - [VisitWebpageTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.VisitWebpageTool)
- **Code Execution**: Dynamic execution of Python code for computational tasks.
  - [PythonInterpreterTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.PythonInterpreterTool)
- **User Interaction**: Enable Human-in-the-Loop collaboration between agents and users.
  - [UserInputTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.UserInputTool): Collect input from users.
- **Speech Processing**: Convert audio to textual data.
  - [SpeechToTextTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.SpeechToTextTool)
- **Workflow Control**: Manage and direct the flow of agent operations.
  - [FinalAnswerTool](/docs/smolagents/v1.26.0/en/reference/default_tools#smolagents.FinalAnswerTool): Conclude agent workflow with final response.

## ApiWebSearchTool[[smolagents.ApiWebSearchTool]]

#### smolagents.ApiWebSearchTool[[smolagents.ApiWebSearchTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L249)

Web search tool that performs API-based searches.
By default, it uses the Brave Search API.

This tool implements a rate limiting mechanism to ensure compliance with API usage policies.
By default, it limits requests to 1 query per second.

Examples:
```python
>>> from smolagents import ApiWebSearchTool
>>> web_search_tool = ApiWebSearchTool(rate_limit=50.0)
>>> results = web_search_tool("Hugging Face")
>>> print(results)
```

**Parameters:**

endpoint (`str`) : API endpoint URL. Defaults to Brave Search API.

api_key (`str`) : API key for authentication.

api_key_name (`str`) : Environment variable name containing the API key. Defaults to "BRAVE_API_KEY".

headers (`dict`, *optional*) : Headers for API requests.

params (`dict`, *optional*) : Parameters for API requests.

rate_limit (`float`, default `1.0`) : Maximum queries per second. Set to `None` to disable rate limiting.

## DuckDuckGoSearchTool[[smolagents.DuckDuckGoSearchTool]]

#### smolagents.DuckDuckGoSearchTool[[smolagents.DuckDuckGoSearchTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L104)

Web search tool that performs searches using the DuckDuckGo search engine.

Examples:
```python
>>> from smolagents import DuckDuckGoSearchTool
>>> web_search_tool = DuckDuckGoSearchTool(max_results=5, rate_limit=2.0)
>>> results = web_search_tool("Hugging Face")
>>> print(results)
```

**Parameters:**

max_results (`int`, default `10`) : Maximum number of search results to return.

rate_limit (`float`, default `1.0`) : Maximum queries per second. Set to `None` to disable rate limiting.

- ****kwargs** : Additional keyword arguments for the `DDGS` client.

## FinalAnswerTool[[smolagents.FinalAnswerTool]]

#### smolagents.FinalAnswerTool[[smolagents.FinalAnswerTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L83)

## GoogleSearchTool[[smolagents.GoogleSearchTool]]

#### smolagents.GoogleSearchTool[[smolagents.GoogleSearchTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L162)

## PythonInterpreterTool[[smolagents.PythonInterpreterTool]]

#### smolagents.PythonInterpreterTool[[smolagents.PythonInterpreterTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L39)

## SpeechToTextTool[[smolagents.SpeechToTextTool]]

#### smolagents.SpeechToTextTool[[smolagents.SpeechToTextTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L646)

## UserInputTool[[smolagents.UserInputTool]]

#### smolagents.UserInputTool[[smolagents.UserInputTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L93)

## VisitWebpageTool[[smolagents.VisitWebpageTool]]

#### smolagents.VisitWebpageTool[[smolagents.VisitWebpageTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L491)

## WebSearchTool[[smolagents.WebSearchTool]]

#### smolagents.WebSearchTool[[smolagents.WebSearchTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L342)

search_exasmolagents.WebSearchTool.search_exahttps://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L455[{"name": "query", "val": ": str"}]
Search using the Exa API. Requires an EXA_API_KEY environment variable.

## WikipediaSearchTool[[smolagents.WikipediaSearchTool]]

#### smolagents.WikipediaSearchTool[[smolagents.WikipediaSearchTool]]

[Source](https://github.com/huggingface/smolagents/blob/v1.26.0/src/smolagents/default_tools.py#L547)

Search Wikipedia and return the summary or full text of the requested article, along with the page URL.

Example:
```python
>>> from smolagents import CodeAgent, InferenceClientModel, WikipediaSearchTool
>>> agent = CodeAgent(
>>>     tools=[
>>>            WikipediaSearchTool(
>>>                user_agent="MyResearchBot (myemail@example.com)",
>>>                language="en",
>>>                content_type="summary",  # or "text"
>>>                extract_format="WIKI",
>>>            )
>>>        ],
>>>     model=InferenceClientModel(),
>>> )
>>> agent.run("Python_(programming_language)")
```

**Parameters:**

user_agent (`str`) : Custom user-agent string to identify the project. This is required as per Wikipedia API policies. See: https://foundation.wikimedia.org/wiki/Policy:Wikimedia_Foundation_User-Agent_Policy

language (`str`, default `"en"`) : Language in which to retrieve Wikipedia article. See: http://meta.wikimedia.org/wiki/List_of_Wikipedias

content_type (`Literal["summary", "text"]`, default `"text"`) : Type of content to fetch. Can be "summary" for a short summary or "text" for the full article.

extract_format (`Literal["HTML", "WIKI"]`, default `"WIKI"`) : Extraction format of the output. Can be `"WIKI"` or `"HTML"`.
