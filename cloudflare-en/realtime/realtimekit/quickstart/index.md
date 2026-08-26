---
description: Set up RealtimeKit in your application with API tokens, SDK installation, and your first meeting.
title: Quickstart
image: https://developers.cloudflare.com/og-docs.png
---

[Skip to content](#main-content)

> Documentation Index  
> Fetch the complete documentation index at: https://developers.cloudflare.com/realtime/llms.txt  
> Use this file to discover all available pages before exploring further.

# Quickstart

Last updated Apr 21, 2026|Copy as Markdown|[View as Markdown](https://developers.cloudflare.com/realtime/realtimekit/quickstart/index.md)|[Agent setup](https://developers.cloudflare.com/agent-setup/)

### Prerequisites

To integrate RealtimeKit in your application, you must have a [Cloudflare account ↗](https://dash.cloudflare.com).

1. Follow the [Create API token guide](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) to create a new token via the [Cloudflare dashboard ↗](https://dash.cloudflare.com/profile/api-tokens).
2. When configuring permissions, ensure that **Realtime** / **Realtime Admin** permissions are selected.
3. Configure any additional [access policies and restrictions](https://developers.cloudflare.com/fundamentals/api/reference/permissions/) as needed for your use case.

_Optional:_ Alternatively, [create tokens programmatically via the API](https://developers.cloudflare.com/fundamentals/api/how-to/create-via-api/). Please ensure your access policy includes the **Realtime** permission.

### Installation

Select a framework based on the platform you are building for.

WebMobile

ReactWeb ComponentsAngular

Please install the following dependencies into your project repository:

```bash
npm i @cloudflare/realtimekit-react @cloudflare/realtimekit-react-ui
```

_Optional:_ You can also build on top of our ready-made template:

```bash
git clone https://github.com/cloudflare/realtimekit-web-examples.git
cd realtimekit-web-examples/react-examples/examples/default-meeting-ui
```

Please install the following dependencies into your project repository:

```bash
npm i @cloudflare/realtimekit-web @cloudflare/realtimekit-ui
```

_Optional:_ You can also build on top of our ready-made template:

```bash
git clone https://github.com/cloudflare/realtimekit-web-examples.git
cd realtimekit-web-examples/html-examples/examples/default-meeting-ui
```

Please install the following dependencies into your project repository:

```bash
npm i @cloudflare/realtimekit-angular @cloudflare/realtimekit-angular-ui
```

_Optional:_ You can also build on top of our ready-made template:

```bash
git clone https://github.com/cloudflare/realtimekit-web-examples.git
cd realtimekit-web-examples/angular-examples/examples/default-meeting-ui
```

Add the following dependency to your `build.gradle` file:

```java
dependencies {
  implementation 'com.cloudflare.realtimekit:ui-android:0.3.0'
}
```

Install the RealtimeKit UI Kit using Swift Package Manager:

1. In Xcode, go to **File > Add Package Dependencies**.
2. Enter the package URL: `https://github.com/dyte-in/RealtimeKitUI`.
3. Select the version and add the package to your project.

Add the following entries to the `Info.plist` file. This gives your app permissions to access the camera and microphone, access photos, and install the required fonts and icons.

```xml
<key>NSBluetoothPeripheralUsageDescription</key>
<string>Access Bluetooth to connect to headphones and audio devices during calls.</string>
<key>NSBluetoothAlwaysUsageDescription</key>
<string>Access Bluetooth to connect to headphones and audio devices during calls.</string>
<key>NSCameraUsageDescription</key>
<string>Access camera to enable video during meetings.</string>
<key>NSMicrophoneUsageDescription</key>
<string>Access microphone to enable audio during meetings.</string>
<key>NSPhotoLibraryUsageDescription</key>
<string>Access photos to share images during meetings.</string>
<key>UIBackgroundModes</key>
<array>
  <string>audio</string>
  <string>voip</string>
  <string>fetch</string>
  <string>remote-notification</string>
</array>
```

The `UIBackgroundModes` key is used in the `Info.plist` file of an iOS app to declare the app's supported background execution modes. This key is an array of strings that specifies the types of background tasks that the app supports. By declaring the background modes, the app can continue to run in the background and perform specific tasks even when it is not in the foreground.

Note

The use of background modes should be justified and comply with Apple's App Store Review Guidelines. Apps that misuse background modes or unnecessarily run in the background may be rejected during the app review process.

Source: [Apple Developer Documentation: Declaring Your App's Supported Background Tasks ↗](https://developer.apple.com/documentation/xcode/configuring-background-execution-modes)

**Minimum requirements:** React Native 0.84 or above, React 19 or above.

Install the dependencies:

```bash
npm install @cloudflare/realtimekit-react-native @cloudflare/react-native-webrtc @cloudflare/realtimekit-react-native-ui @react-native-documents/picker react-native-file-viewer react-native-fs react-native-sound-player react-native-webview react-native-svg@^15.13.0 react-native-safe-area-context@^5.0.0
```

**Minimum Expo version:** 56 or above.

Install the dependencies:

```bash
npx expo install @cloudflare/realtimekit-react-native-ui @cloudflare/realtimekit-react-native @cloudflare/react-native-webrtc @react-native-documents/picker react-native-file-viewer react-native-fs react-native-sound-player react-native-webview react-native-svg@^15.13.0 react-native-safe-area-context@^5.0.0
```

Install Expo config plugins:

```bash
npx expo install @expo/config-plugins
```

Add the plugins to your `app.json`:

```json
{
	"expo": {
		"plugins": [
			"@cloudflare/realtimekit-react-native",
			"@cloudflare/react-native-webrtc"
		]
	}
}
```

Run `prebuild` to set up native modules:

```bash
npx expo prebuild
```

The following instructions are for release builds. Debug builds should work without additional steps.

Edit your `android/gradle.properties` and add the following lines:

```txt
android.useFullClasspathForDexingTransform=true
```

Add a required `blob_provider_authority` string resource in the `strings.xml` file:

```xml
<resources>
  ...
  <string name="blob_provider_authority">YOUR_APP_RESOURCE_NAME</string>
  ...
</resources>
```

Create or append to the file `android/app/proguard-rules.pro`:

```txt
-keep class realtimekit.org.webrtc.** { *; }
-dontwarn org.chromium.build.BuildHooksAndroid
```

In your `android/app/build.gradle`, edit the release configuration and add the following line importing the ProGuard configuration:

```java
buildTypes {
  release {
    ...
    proguardFiles getDefaultProguardFile('proguard-android.txt'), 'proguard-rules.pro'
  }
}
```

#### Background audio and video (Android)

The SDK includes an Android foreground service that keeps audio and video running when the app moves to the background. The service starts automatically when a participant joins a meeting and stops when they leave.

By default the service is enabled with generic notification text. To customize the notification or disable the service, pass `keepAliveService` to the `useRealtimeKitClient` hook before calling `initMeeting`:

```tsx
const [meeting, initMeeting] = useRealtimeKitClient({
  keepAliveService: {
    enabled: true,                        // set to false to disable entirely
    title: "Team call",                   // notification title
    text: "Tap to return to your meeting", // notification body
  },
});
```

On Android 13 and above, the SDK automatically requests the `POST_NOTIFICATIONS` permission so the notification appears in the shade. No additional setup is required.

**Minimum supported iOS version: 15.1.**

Open your `Podfile` and set the platform to iOS 15.1:

```txt
platform :ios, '15.1'
```

Add the following permission entries to your `Info.plist` file:

```xml
<key>NSCameraUsageDescription</key>
<string>Access camera to enable video during meetings.</string>
<key>NSMicrophoneUsageDescription</key>
<string>Access microphone to enable audio during meetings.</string>
<key>NSPhotoLibraryUsageDescription</key>
<string>Access photos to share images during meetings.</string>
<key>UIViewControllerBasedStatusBarAppearance</key>
<false/>
```

#### Screen sharing (iOS)

iOS screen sharing requires a Broadcast Upload Extension and additional native setup. Refer to the [Screen Share Setup (iOS)](https://developers.cloudflare.com/realtime/realtimekit/core/local-participant/#screen-share-setup-ios) guide for full instructions.

Once native setup is complete, pass `iOSScreenshareEnabled={true}` to `RtkMeeting` to enable the screen share button in the UI:

```tsx
<RtkMeeting meeting={meeting} iOSScreenshareEnabled={true} />
```

### Create a RealtimeKit App

You can create an application from the [Cloudflare Dashboard ↗](https://dash.cloudflare.com/?to=/:account/realtime/kit), by clicking on Create App.

_Optional:_ You can also use our [API reference](https://developers.cloudflare.com/api/resources/realtime%5Fkit/) for creating an application:

```bash
curl --location 'https://api.cloudflare.com/client/v4/accounts/<account_id>/realtime/kit/apps' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer <api_token>' \
--data '{"name": "My First Cloudflare RealtimeKit app"}'
```

> **Note:** We recommend creating different apps for staging and production environments.

### Create a Meeting

Use our [Meetings API](https://developers.cloudflare.com/api/resources/realtime%5Fkit/subresources/meetings/methods/create/) to create a meeting. We will use the **ID from the response** in subsequent steps.

```bash
curl --location 'https://api.cloudflare.com/client/v4/accounts/<account_id>/realtime/kit/<app_id>/meetings' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer <api_token>' \
--data '{"title": "My First Cloudflare RealtimeKit meeting"}'
```

### Add Participants

#### Create a Preset

Presets define what permissions a user should have. Learn more in the Concepts guide. You can create new presets using the [Presets API](https://developers.cloudflare.com/api/resources/realtime%5Fkit/subresources/presets/methods/create/) or via the [RealtimeKit dashboard ↗](https://dash.cloudflare.com/?to=/:account/realtime/kit).

> **Note:** Skip this step if you created the app in the dashboard—default presets are already set up for you.

> **Note:** Presets can be reused across multiple meetings. Define a role (for example, admin or viewer) once and apply it to participants in any number of meetings.

#### Add a Participant

A participant is added to a meeting using the `Meeting ID` created above and selecting a `Preset Name` from the available options.

The response includes an `authToken` which the **Client SDK uses to add this participant to the meeting** room.

```bash
curl --location 'https://api.cloudflare.com/client/v4/accounts/<account_id>/realtime/kit/<app_id>/meetings/<meeting_id>/participants' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer <api_token>' \
--data '{
  "name": "Mary Sue",
  "preset_name": "<preset_name>",
  "custom_participant_id": "<uuid_of_the_user_in_your_system>"
}'
```

Learn more about adding participants in the [API reference](https://developers.cloudflare.com/api/resources/realtime%5Fkit/subresources/meetings/methods/add%5Fparticipant/).

### Frontend Integration

You can now add the RealtimeKit Client SDK to your application.

Inside your react application, add the following code:

```ts

import { useEffect } from "react";
import {
	useRealtimeKitClient,
	useRealtimeKitMeeting,
	RealtimeKitProvider,
} from "@cloudflare/realtimekit-react";
import { RtkMeeting } from "@cloudflare/realtimekit-react-ui";

export default function App() {
  const [meeting, initMeeting] = useRealtimeKitClient();

useEffect(() => {
initMeeting({ authToken: '<auth-token>' });
}, []);

return (

<RealtimeKitProvider value={meeting}>
	<MyMeetingUI />
</RealtimeKitProvider>
); }

export default function MyMeetingUI() {
  const { meeting } = useRealtimeKitMeeting();
  return (
    <RtkMeeting mode="fill" meeting={meeting} showSetupScreen={true} />
  );

}
```

Replace `<auth-token>` with the authToken obtained from the previous step.

Run the application and navigate to the meeting page to see the RealtimeKit Client SDK in action.

```bash
npm run dev
```

_Optional:_ If you are using our ready-made template, run the following command to start the application:

```bash
npm i -g vite && npm run dev
```

Open the app in your browser. To join the meeting, append your auth token to the preview URL:

```bash
http://localhost:5173?authToken=<auth_token>
```

Inside your html application, add the following code:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Default Meeting UI | RealtimeKit</title>

    <!-- Import helper to load UI Kit components -->
    <script type="module">
      import { defineCustomElements } from 'https://cdn.jsdelivr.net/npm/@cloudflare/realtimekit-ui@latest/loader/index.es2017.js';
      defineCustomElements();
    </script>

    <!-- Import RealtimeKit Core via CDN -->
    <script src="https://cdn.jsdelivr.net/npm/@cloudflare/realtimekit@latest/dist/browser.js"></script>

  </head>
  <body>
    <rtk-meeting id="my-meeting" show-setup-screen="true" />

    <script>
      const searchParams = new URL(window.location.href).searchParams;

      const authToken = searchParams.get('authToken');

      if (!authToken) {
        alert(
          "An authToken wasn't passed, please pass an authToken in the URL query to join a meeting."
        );
      }

      // Initialize a meeting
      RealtimeKitClient.init({
        authToken,
      }).then((meeting) => {
        document.getElementById('my-meeting').meeting = meeting;
      });
    </script>

  </body>
</html>
```

Replace `<auth-token>` with the authToken obtained from the previous step.

Run the application and navigate to the meeting page to see the RealtimeKit Client SDK in action.

```bash
npm run dev
```

_Optional:_ If you are using our ready-made template, run the following command to start the application:

```bash
npm i -g vite && npm run dev
```

Open the app in your browser. To join the meeting, append your auth token to the preview URL:

```bash
http://localhost:5173?authToken=<auth_token>
```

Load the RTKComponentsModule into your app module. This is typically the app.module.ts file. This allows you to use the UI components in your component HTML files.

```ts
import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RTKComponentsModule } from '@cloudflare/realtimekit-angular';
import { AppComponent } from './app.component';

@NgModule({
declarations: [AppComponent],
imports: [BrowserModule, RTKComponentsModule],
providers: [],
bootstrap: [AppComponent],
})
export class AppModule {};
```

_Optional:_ If you are using TypeScript, set allowSyntheticDefaultImports as true in your tsconfig.json.

```ts
{
	"compilerOptions": {
		"allowSyntheticDefaultImports": true
	}
}
```

Load the RtkMeeting component to your template file (component.html).

```html
<rtk-meeting #myid></rtk-meeting>
```

Initialise the Meeting

```ts
	class AppComponent {
	title = 'MyProject';
	@ViewChild('myid') meetingComponent: RtkMeeting;
	rtkMeeting: RealtimeKitClient;

	async ngAfterViewInit() {
		const meeting = await RealtimeKitClient.init({
		authToken: '<auth-token>',
		});
		meeting.join();
		this.rtkMeeting = meeting;
		if (this.meetingComponent) this.meetingComponent.meeting = meeting;
	}
	}
```

Replace `<auth-token>` with the authToken obtained from the previous step.

Run the application and navigate to the meeting page to see the RealtimeKit Client SDK in action.

```bash
npm run dev
```

_Optional:_ If you are using our ready-made template, run the following command to start the application:

```bash
npm i -g vite && npm run dev
```

Open the app in your browser. To join the meeting, append your auth token to the preview URL:

```bash
http://localhost:5173?authToken=<auth_token>
```

Was this helpful?

YesNo

## On this page

[![](https://developers.cloudflare.com/_astro/logo.te5VL_aD.svg)Docs](https://developers.cloudflare.com/)

```json
{"@context":"https://schema.org","@type":"WebPage","@id":"https://developers.cloudflare.com/realtime/realtimekit/quickstart/#page","headline":"Quickstart · Cloudflare Realtime docs","description":"Set up RealtimeKit in your application with API tokens, SDK installation, and your first meeting.","url":"https://developers.cloudflare.com/realtime/realtimekit/quickstart/","inLanguage":"en","image":"https://developers.cloudflare.com/og-docs.png","dateModified":"2026-04-21","publisher":{"@type":"Organization","name":"Cloudflare","description":"One platform for your apps, agents, and workforce. Build, secure, and scale without managing infrastructure","url":"https://www.cloudflare.com/","sameAs":["https://github.com/cloudflare","https://www.linkedin.com/company/cloudflare","https://x.com/cloudflare"],"logo":{"@type":"ImageObject","url":"https://developers.cloudflare.com/logo.svg"},"address":{"@type":"PostalAddress","streetAddress":"101 Townsend St","addressLocality":"San Francisco","addressRegion":"CA","postalCode":"94107","addressCountry":"US"},"contactPoint":[{"@type":"ContactPoint","contactType":"Customer Support","url":"https://support.cloudflare.com/","availableLanguage":["English"]},{"@type":"ContactPoint","contactType":"Sales","url":"https://www.cloudflare.com/contact/","availableLanguage":["English"]}]},"isPartOf":{"@type":"WebSite","@id":"https://developers.cloudflare.com/#website","name":"Cloudflare Docs","url":"https://developers.cloudflare.com/"}}
```
