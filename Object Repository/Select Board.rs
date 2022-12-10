<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Select Board</name>
   <tag></tag>
   <elementGuidId>8da71659-1150-4bd9-b41b-1013e82be931</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;limitList\u003d10\u0026limitCard\u003d10\u0026teamId\u003d637bac8166ccc7048fda8209\u0026companyId\u003d63779a18b24aac5c973a0fd2&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>3f25afa7-1447-4e64-9b85-3d034f6ab932</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYzNDZjNDg1NTc1NzdhMWM5NDhhODU1MSIsImdvb2dsZUlkIjoiMTE3NjQ1MDEyMjQ3NTM1NzkyMjM5IiwiZW1haWwiOiJuY2FoMTdAZ21haWwuY29tIiwiZnVsbE5hbWUiOiJuY2FoIDE3IiwicGhvdG9VcmwiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BTG01d3UzT0o3cjU5R1B5czZvMGVUSGxEMzkzR3d0MFlwTmdqenZTUUY0Sj1zOTYtYyIsImJpbyI6IiIsInN0YXR1cyI6IiIsImRlZmF1bHRDb21wYW55IjpudWxsLCJjcmVhdGVkQXQiOiIyMDIyLTEwLTEyVDEzOjQzOjMzLjIwOFoiLCJ1cGRhdGVkQXQiOiIyMDIyLTEwLTIxVDA3OjU1OjAxLjg4NloiLCJfX3YiOjB9LCJpYXQiOjE2Njg3ODI1MDYsImV4cCI6MTY3MTM3NDUwNn0.dL046qtyHaJdcMemU83Q1-uf8FvTPJEoPM1i0eks5bE</value>
      <webElementGuid>601b0b08-924d-4304-950f-64878a146939</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://stagingapi.cicle.app/v2/boards/637bac8166ccc7048fda820d?limitList=10&amp;limitCard=10&amp;teamId=637bac8166ccc7048fda8209&amp;companyId=63779a18b24aac5c973a0fd2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
