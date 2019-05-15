<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateSubscription</name>
   <tag></tag>
   <elementGuidId>3f9b5c86-a205-4782-8cf7-f3db4ee77813</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot; standalone=&quot;no&quot;?>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:aom=&quot;http://www.i2i.com/fcbs/soa/schemas/AOMWS&quot;>
  &lt;soapenv:Header/>
  &lt;soapenv:Body>
     &lt;aom:CreateSubscription>
               &lt;pReqInfo>
          &lt;ACTION_INTERACTION_TYPE_ID>145&lt;/ACTION_INTERACTION_TYPE_ID>
          &lt;ACTION_DATE>2019/03/26-18:01:00&lt;/ACTION_DATE>
          &lt;TRANSACTION_ID>TLR-dc55d895-737d-458a-bb6c-2206fc4762d0&lt;/TRANSACTION_ID>
          &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
          &lt;USER_NAME>BSCS&lt;/USER_NAME>
          &lt;CHANNEL_NAME>BSCS&lt;/CHANNEL_NAME>
          &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
          &lt;MODIFY_INFO>
             &lt;CREATE_INFO>
                &lt;CREATE_DATE>2019/03/18-11:52:14&lt;/CREATE_DATE>
                &lt;CREATE_USER>BSCS&lt;/CREATE_USER>
             &lt;/CREATE_INFO>
          &lt;/MODIFY_INFO>
       &lt;/pReqInfo>
        &lt;pSubscInfo>
           &lt;SUBSCRIPTION_KEY>
              &lt;!--You have a CHOICE of the next 2 items at this level-->
              &lt;SUBSCRIPTION_ID>${subID}&lt;/SUBSCRIPTION_ID>
           &lt;/SUBSCRIPTION_KEY>
           &lt;SUBSCRIPTION_DEFINITION>
              &lt;!--Optional:-->
              &lt;BILLING_ACCOUNT_ID>${billAcctID}&lt;/BILLING_ACCOUNT_ID>
              &lt;SUBSCRIPTION_NAME>ctezcan&lt;/SUBSCRIPTION_NAME>
              &lt;!--Optional:-->
              &lt;SUBSCRIPTION_PLAN_CODE>2114&lt;/SUBSCRIPTION_PLAN_CODE>
              &lt;SUBSCRIPTION_STATUS>A&lt;/SUBSCRIPTION_STATUS>
           &lt;/SUBSCRIPTION_DEFINITION>
           &lt;!--Zero or more repetitions:-->
        &lt;/pSubscInfo>
     &lt;/aom:CreateSubscription>
  &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CreateSubscription</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>f347cd4c-376b-4699-b27e-21323707e04f</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.subID</defaultValue>
      <description></description>
      <id>c75b50e7-0c03-471a-94c7-e5feee655fc6</id>
      <masked>false</masked>
      <name>subID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.billAcctID</defaultValue>
      <description></description>
      <id>1c099c34-b6b9-4156-a4d3-b2250c463de8</id>
      <masked>false</masked>
      <name>billAcctID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://172.30.10.32:8181/AOMWSSubscription/AOMWSSubscription?wsdl</wsdlAddress>
</WebServiceRequestEntity>
