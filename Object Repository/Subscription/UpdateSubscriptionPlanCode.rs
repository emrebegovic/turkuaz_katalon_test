<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateSubscriptionPlanCode</name>
   <tag></tag>
   <elementGuidId>d6c7b111-808a-47e9-a6c0-869319f4db8b</elementGuidId>
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
      &lt;aom:UpdateSubscriptionPlanCode>
         &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>2&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_REASON_CODE>PORT_IN&lt;/ACTION_REASON_CODE>
            &lt;ACTION_DATE>2019/04/10-00:00:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>YY1651561&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;USER_NAME>BSCS&lt;/USER_NAME>
            &lt;CHANNEL_NAME>BSCS&lt;/CHANNEL_NAME>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2019/04/10-00:00:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>BSCS&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pKey>
            &lt;SUBSCRIPTION_ID>${subID}&lt;/SUBSCRIPTION_ID>
         &lt;/pKey>
         &lt;SUBSCRIPTION_PLAN_CODE>${subPlanCode}&lt;/SUBSCRIPTION_PLAN_CODE>
      &lt;/aom:UpdateSubscriptionPlanCode>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>UpdateSubscriptionPlanCode</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.subID</defaultValue>
      <description></description>
      <id>26bc73b1-4501-4f36-9ed7-8b38334461b4</id>
      <masked>false</masked>
      <name>subID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.subPlanCode</defaultValue>
      <description></description>
      <id>3ebcc609-65c2-4bd8-add3-1d1068367955</id>
      <masked>false</masked>
      <name>subPlanCode</name>
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
