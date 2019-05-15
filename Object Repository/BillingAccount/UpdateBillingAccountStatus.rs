<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateBillingAccountStatus</name>
   <tag></tag>
   <elementGuidId>9f251fb0-20dc-4cdc-a29d-5ea783d12aba</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:aom=&quot;http://www.i2i.com/fcbs/soa/schemas/AOMWS&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;aom:UpdateBillingAccountStatus>
           &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>143&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_DATE>2019/02/01-08:01:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>CA147852&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>5009010&lt;/CUSTOMER_ID>
            &lt;USER_NAME>cansuarslan&lt;/USER_NAME>
            &lt;CHANNEL_NAME>cansuarslan&lt;/CHANNEL_NAME>
            &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2019/02/01-08:01:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>cansuarslan&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pKey>
            &lt;BILLING_ACCOUNT_ID>${billAcctID}&lt;/BILLING_ACCOUNT_ID>
         &lt;/pKey>
         &lt;pStatus>${baStatus}&lt;/pStatus>
      &lt;/aom:UpdateBillingAccountStatus>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>UpdateBillingAccountStatus</soapServiceFunction>
   <variables>
      <defaultValue>'D'</defaultValue>
      <description></description>
      <id>3060b0c4-b2a1-4483-8649-39917cafaa5b</id>
      <masked>false</masked>
      <name>baStatus</name>
   </variables>
   <variables>
      <defaultValue>'1257043'</defaultValue>
      <description></description>
      <id>e4db9efd-b351-49c4-b2d9-9a84fe727c79</id>
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
   <wsdlAddress>http://172.30.10.32:8181/AOMWSBillingAccount/AOMWSBillingAccount?wsdl</wsdlAddress>
</WebServiceRequestEntity>
