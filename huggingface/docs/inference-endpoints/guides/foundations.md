# Foundations

The Inference Endpoints dashboard is the central interface to manage, monitor, and deploy inference endpoints across
multiple organizations and accounts. Users can switch between organizations, view endpoint statuses, manage quotas, and
access deployment configurations. You can access the dashboard by logging in on [endpoints.huggingface.co](https://endpoints.huggingface.co)

## Managing Endpoints

### Creating New Endpoints
Click the + New button in the top section to create a new endpoint deployment. This will take you to the Model Catalog which
provides access to 100+ pre-configured models available for deployment as inference endpoints. Use this to browse,
filter, and deploy models directly.

![new](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/1-new.png)

If you cannot find a suitable model in the catalog you can click the "Deploy From Hugging Face" button which allows you to deploy from
any Hugging Face repository.

![catalog](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/2-catalog.png)

After this you will be directed to the configuration page. You can read [here](./configuration) more in detail about all the configuration options.

### Endpoint States
Endpoints can be in one of several states:
- **Running**: Endpoint is ready to serve requests
- **Initializing**: Endpoint is starting up 
- **Paused**: Endpoint has been stopped, which counts towards your quota
- **Scaled to Zero**: Endpoint is idle and consuming no compute resources
- **Failed**: Endpoint encountered an error and is not operational

### Managing existing endpoints

The endpoint details page provides information and lets you control the configuration of an individual endpoint.
Access this view by clicking on any endpoint from the main endpoints list.

The endpoint name displays with its current state. You can pause a running endpoint or wake up an endpoint scaled to zero.

- **Overview**: Current status and configuration summary
- **Analytics**: Performance metrics and usage statistics, for more in-depth reading please [visit here](./analytics)
- **Logs**: Runtime logs and debugging information, more in-depth docs can be found [here](./logs)
- **Usage & Cost**: Billing information and resource consumption
- **Settings**: Configuration and management options

The page displays the configuration options that are available for each endpoint. You'll find a more in-depth walk-through of all options under
the [configuration section](./configuration)

![endpoint](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/8-endpoint.png)

## Using the Dashboard

### Viewing Endpoint Information
The endpoints table displays critical information for each deployment. Click Edit Columns to show or hide specific
information columns. Available columns include State, Task, Instance, Vendor, Container, Access, Tags, URL, Created, and Updated timestamps

![list](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/3-list.png)

### Filtering and Search
Use the search bar to filter endpoints by name, provider, task, or tags.
The Status dropdown allows filtering by specific endpoint states.

![filter](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/4-filter.png)

### Account Management
Access account settings through the dropdown menu in the top-right corner. This provides access to organization switching,
billing information, and access token management.

![account](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/5-account.png)

## Quotas
The Quotas section displays your current resource usage and limits across different cloud providers and hardware types.
Access this view to monitor consumption and request additional capacity when needed.

Note that:
- *Paused* endpoints will not count against 'used' quota.
- *Scaled to Zero* endpoints will be counted as 'used' quota—simply pause the scaled-to-zero endpoint if you would like to unlock this quota. 

![quotas](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/6-quotas.png)

### Requesting Additional Quota
Use the Request More button to submit requests for increased limits when approaching quota thresholds. This allows you to
scale your inference deployments beyond current allocations. Or click the button below:

Request More

## Audit Logs
The Audit Logs section provides a chronological record of all actions performed on your inference endpoints. You can use this
to track changes, troubleshoot issues, and maintain security oversight of your deployments.

Use the All Endpoints dropdown to filter logs by specific endpoint instances. This allows you to focus on activity for particular
deployments.

![audit](https://raw.githubusercontent.com/huggingface/hf-endpoints-documentation/main/assets/foundations/7-audit.png)

### Log Entry Structure
Each audit log entry contains:
- **User Avatar and name**
- **Action Type**: Type of operation performed (resumed, updated etc.)
- **Endpoint Name**
- **Timestamp**
- **Action Details**:
    - Instance Changes: For example hardware scaling modifications
    - Configuration Updates: Parameter adjustments
    - State Changes: Operational status modifications
- **Request Metadata**: Technical details for troubleshooting:
    - IP Address: Source IP of the request
    - X-Request-Id: Unique identifier for tracking API calls
