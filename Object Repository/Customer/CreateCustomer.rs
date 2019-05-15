<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateCustomer</name>
   <tag></tag>
   <elementGuidId>1b009e64-ca55-4dbd-843b-5ad680e1a6a9</elementGuidId>
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
      &lt;aom:CreateCustomer>
          &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>143&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_DATE>2019/01/01-00:00:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>CA123456&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>990001001&lt;/CUSTOMER_ID>
            &lt;USER_NAME>cansuarslan&lt;/USER_NAME>
            &lt;CHANNEL_NAME>test&lt;/CHANNEL_NAME>
            &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2017/12/17-10:00:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>cansuarslan&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pCustInfo>
            &lt;CUSTOMER_KEY>
               &lt;!--You have a CHOICE of the next 2 items at this level-->
               &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;/CUSTOMER_KEY>
            &lt;CUSTOMER_DEFINITION>
               &lt;CUSTOMER_NAME>cansuarslan&lt;/CUSTOMER_NAME>
               &lt;CUSTOMER_STATUS>A&lt;/CUSTOMER_STATUS>
               &lt;CUSTOMER_TYPE>I&lt;/CUSTOMER_TYPE>
               &lt;SEGMENT_CODE>123&lt;/SEGMENT_CODE>
            &lt;/CUSTOMER_DEFINITION>
         &lt;/pCustInfo>
      &lt;/aom:CreateCustomer>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CreateCompositeCustomer</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>b57205fb-3078-4516-8eee-cd8be2103c44</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testdata.TestDataFactory.findTestData
import com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable





RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getResponseText()).contains('Katalon Test Project')


WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))


</verificationScript>
   <wsdlAddress>http://172.30.10.32:8181/AOMWSCustomer/AOMWSCustomer?wsdl</wsdlAddress>
</WebServiceRequestEntity>
