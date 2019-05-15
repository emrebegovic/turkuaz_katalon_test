<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>NewSalesOrder</name>
   <tag></tag>
   <elementGuidId>0bc15e24-1b3a-4db2-af15-c3c79e30a5c7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:aom=&quot;http://www.i2i.com/fcbs/soa/schemas/AOMWS&quot; xmlns:fn=&quot;http://www.w3.org/2005/xpath-functions&quot; xmlns:xs=&quot;http://www.w3.org/2001/XMLSchema&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;aom:NewSalesOrder>
         &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>2&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_REASON_CODE>PORT_IN&lt;/ACTION_REASON_CODE>
            &lt;ACTION_DATE>${nsoDate}&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>YY1600561&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;USER_NAME>BSCS&lt;/USER_NAME>
            &lt;CHANNEL_NAME>BSCS&lt;/CHANNEL_NAME>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>${nsoDate}&lt;/CREATE_DATE>
                  &lt;CREATE_USER>BSCS&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pSalesOrder>
            &lt;SALES_ORDER_ID>138&lt;/SALES_ORDER_ID>
            &lt;SALES_ORDER_ITEM>
               &lt;!--voice usage-->
               &lt;SALES_ORDER_ITEM_ID>421&lt;/SALES_ORDER_ITEM_ID>
               &lt;ACTION_DATE>${nsoDate}&lt;/ACTION_DATE>
               &lt;SERVICE>
                  &lt;SERVICE_ID>421&lt;/SERVICE_ID>
                  &lt;SERVICE_NAME>VOICE SERVICE&lt;/SERVICE_NAME>
                  &lt;SERVICE_SPECIFICATION_ID>185&lt;/SERVICE_SPECIFICATION_ID>
                  &lt;NETWORK_SERVICE_CODE>VOICE&lt;/NETWORK_SERVICE_CODE>
               &lt;/SERVICE>
               &lt;PRODUCT>
                  &lt;PRODUCT_KEY>
                     &lt;PRODUCT_ID>${voiceUsageProductID}&lt;/PRODUCT_ID>
                  &lt;/PRODUCT_KEY>
                  &lt;PRODUCT_DEFINITION>
                     &lt;PRODUCT_NAME>VOICE&lt;/PRODUCT_NAME>
                     &lt;DESCRIPTION>VOICE&lt;/DESCRIPTION>
                     &lt;PRODUCT_STATUS>A&lt;/PRODUCT_STATUS>
                     &lt;PRODUCT_NUMBER>NR-91501421&lt;/PRODUCT_NUMBER>
                     &lt;PRODUCT_SERIAL_NUMBER>SN-91501421&lt;/PRODUCT_SERIAL_NUMBER>
                  &lt;/PRODUCT_DEFINITION>
                  &lt;PRODUCT_PRICE_INFO>
                     &lt;PRODUCT_PRICE_NAME>VOICE SERVICE&lt;/PRODUCT_PRICE_NAME>
                     &lt;PRODUCT_PRICE_DESCR>VOICE SERVICE&lt;/PRODUCT_PRICE_DESCR>
                     &lt;UOM_TYPE_ID>3&lt;/UOM_TYPE_ID>
                     &lt;TARIFF_ID>111301&lt;/TARIFF_ID>
                  &lt;/PRODUCT_PRICE_INFO>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>msisdn&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>${msisdn}&lt;/PRODUCT_CHAR_VALUE>
                     &lt;DESCRIPTION>MSISDN&lt;/DESCRIPTION>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>1&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>1&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>imsi&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>${imsi}&lt;/PRODUCT_CHAR_VALUE>
                     &lt;DESCRIPTION>IMSI&lt;/DESCRIPTION>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>1&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>1&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
               &lt;/PRODUCT>
               &lt;OFFER>
                  &lt;PRODUCT_OFFER_KEY>
                     &lt;PRODUCT_OFFER_ID>421&lt;/PRODUCT_OFFER_ID>
                  &lt;/PRODUCT_OFFER_KEY>
                  &lt;PRODUCT_OFFER_DEFINITION>
                     &lt;OFFER_NAME>Texos&lt;/OFFER_NAME>
                     &lt;OFFER_STATUS>A&lt;/OFFER_STATUS>
                     &lt;IS_BUNDLE>0&lt;/IS_BUNDLE>
                     &lt;OFFER_TYPE>POFFR&lt;/OFFER_TYPE>
                  &lt;/PRODUCT_OFFER_DEFINITION>
               &lt;/OFFER>
               &lt;QUANTITY>1&lt;/QUANTITY>
            &lt;/SALES_ORDER_ITEM>
            &lt;SALES_ORDER_ITEM>
               &lt;!--voice fu-->
               &lt;SALES_ORDER_ITEM_ID>424&lt;/SALES_ORDER_ITEM_ID>
               &lt;ACTION_DATE>${nsoDate}&lt;/ACTION_DATE>
               &lt;REASON_CODE>Real_Sale&lt;/REASON_CODE>
               &lt;SERVICE>
                  &lt;SERVICE_ID>424&lt;/SERVICE_ID>
                  &lt;SERVICE_NAME>100 cansu voice Paketi&lt;/SERVICE_NAME>
                  &lt;DESCRIPTION>Core Service&lt;/DESCRIPTION>
                  &lt;SERVICE_SPECIFICATION_ID>0&lt;/SERVICE_SPECIFICATION_ID>
                  &lt;FCBS_SERVICE_SPECIFICATION_ID>2&lt;/FCBS_SERVICE_SPECIFICATION_ID>
               &lt;/SERVICE>
               &lt;PRODUCT>
                  &lt;PRODUCT_KEY>
                     &lt;PRODUCT_ID>${voiceFUProductID}&lt;/PRODUCT_ID>
                  &lt;/PRODUCT_KEY>
                  &lt;PRODUCT_DEFINITION>
                     &lt;PRODUCT_NAME>100 cansu voice Paketi&lt;/PRODUCT_NAME>
                     &lt;DESCRIPTION>100 cansu VOİCE Paketi&lt;/DESCRIPTION>
                     &lt;PARENT_PRODUCT_ID>${voiceUsageProductID}&lt;/PARENT_PRODUCT_ID>
                     &lt;PRODUCT_STATUS>A&lt;/PRODUCT_STATUS>
                     &lt;PRODUCT_NUMBER>NR-91501424&lt;/PRODUCT_NUMBER>
                     &lt;PRODUCT_SERIAL_NUMBER>SN-91501424&lt;/PRODUCT_SERIAL_NUMBER>
                  &lt;/PRODUCT_DEFINITION>
                  &lt;PRODUCT_PRICE_INFO>
                     &lt;UOM_TYPE_ID>2&lt;/UOM_TYPE_ID>
                     &lt;TARIFF_ID>111302&lt;/TARIFF_ID>
                  &lt;/PRODUCT_PRICE_INFO>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>DISCOUNT_PACK_ID&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>444102&lt;/PRODUCT_CHAR_VALUE>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>0&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>0&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
               &lt;/PRODUCT>
               &lt;OFFER>
                  &lt;PRODUCT_OFFER_KEY>
                     &lt;PRODUCT_OFFER_ID>424&lt;/PRODUCT_OFFER_ID>
                  &lt;/PRODUCT_OFFER_KEY>
                  &lt;PRODUCT_OFFER_DEFINITION>
                     &lt;OFFER_NAME>Core Product&lt;/OFFER_NAME>
                     &lt;OFFER_STATUS>A&lt;/OFFER_STATUS>
                     &lt;IS_BUNDLE>Y&lt;/IS_BUNDLE>
                  &lt;/PRODUCT_OFFER_DEFINITION>
               &lt;/OFFER>
               &lt;QUANTITY>1&lt;/QUANTITY>
            &lt;/SALES_ORDER_ITEM>
      
           
           &lt;SALES_ORDER_ITEM>
               &lt;!--sms usage-->
               &lt;SALES_ORDER_ITEM_ID>419&lt;/SALES_ORDER_ITEM_ID>
               &lt;ACTION_DATE>${nsoDate}&lt;/ACTION_DATE>   &lt;!--2019/04/01-00:00:00-->
               &lt;SERVICE>
                  &lt;SERVICE_ID>419&lt;/SERVICE_ID>
                  &lt;SERVICE_NAME>SMS SEVICE&lt;/SERVICE_NAME>
                  &lt;SERVICE_SPECIFICATION_ID>180&lt;/SERVICE_SPECIFICATION_ID>
                  &lt;NETWORK_SERVICE_CODE>SMS&lt;/NETWORK_SERVICE_CODE>
               &lt;/SERVICE>
               &lt;PRODUCT>
                  &lt;PRODUCT_KEY>
                     &lt;PRODUCT_ID>${smsUsageProductID}&lt;/PRODUCT_ID>
                  &lt;/PRODUCT_KEY>
                  &lt;PRODUCT_DEFINITION>
                     &lt;PRODUCT_NAME>OTHER&lt;/PRODUCT_NAME>
                     &lt;DESCRIPTION>OTHER&lt;/DESCRIPTION>
                     &lt;PRODUCT_STATUS>A&lt;/PRODUCT_STATUS>
                     &lt;PRODUCT_NUMBER>NR-91501419&lt;/PRODUCT_NUMBER>
                     &lt;PRODUCT_SERIAL_NUMBER>SN-91501419&lt;/PRODUCT_SERIAL_NUMBER>
                  &lt;/PRODUCT_DEFINITION>
                  &lt;PRODUCT_PRICE_INFO>
                     &lt;PRODUCT_PRICE_NAME>SMS SEVICE&lt;/PRODUCT_PRICE_NAME>
                     &lt;PRODUCT_PRICE_DESCR>SMS SEVICE&lt;/PRODUCT_PRICE_DESCR>
                     &lt;UOM_TYPE_ID>3&lt;/UOM_TYPE_ID>
                     &lt;TARIFF_ID>111303&lt;/TARIFF_ID>
                  &lt;/PRODUCT_PRICE_INFO>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>msisdn&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>${msisdn}&lt;/PRODUCT_CHAR_VALUE>
                     &lt;DESCRIPTION>MSISDN&lt;/DESCRIPTION>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>1&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>1&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>imsi&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>${imsi}&lt;/PRODUCT_CHAR_VALUE>
                     &lt;DESCRIPTION>IMSI&lt;/DESCRIPTION>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>1&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>1&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
               &lt;/PRODUCT>
               &lt;OFFER>
                  &lt;PRODUCT_OFFER_KEY>
                     &lt;PRODUCT_OFFER_ID>419&lt;/PRODUCT_OFFER_ID>
                  &lt;/PRODUCT_OFFER_KEY>
                  &lt;PRODUCT_OFFER_DEFINITION>
                     &lt;OFFER_NAME>Texos&lt;/OFFER_NAME>
                     &lt;OFFER_STATUS>A&lt;/OFFER_STATUS>
                     &lt;IS_BUNDLE>0&lt;/IS_BUNDLE>
                     &lt;OFFER_TYPE>POFFR&lt;/OFFER_TYPE>
                  &lt;/PRODUCT_OFFER_DEFINITION>
               &lt;/OFFER>
               &lt;QUANTITY>1&lt;/QUANTITY>
            &lt;/SALES_ORDER_ITEM>
             
             
            &lt;SALES_ORDER_ITEM>
               &lt;!--data usage-->
               &lt;SALES_ORDER_ITEM_ID>420&lt;/SALES_ORDER_ITEM_ID>
               &lt;ACTION_DATE>${nsoDate}&lt;/ACTION_DATE>
               &lt;SERVICE>
                  &lt;SERVICE_ID>420&lt;/SERVICE_ID>
                  &lt;SERVICE_NAME>DATA&lt;/SERVICE_NAME>
                  &lt;SERVICE_SPECIFICATION_ID>185&lt;/SERVICE_SPECIFICATION_ID>
                  &lt;NETWORK_SERVICE_CODE>DATA&lt;/NETWORK_SERVICE_CODE>
               &lt;/SERVICE>
               &lt;PRODUCT>
                  &lt;PRODUCT_KEY>
                     &lt;PRODUCT_ID>${dataUsageProductID}&lt;/PRODUCT_ID>
                  &lt;/PRODUCT_KEY>
                  &lt;PRODUCT_DEFINITION>
                     &lt;PRODUCT_NAME>GPRS&lt;/PRODUCT_NAME>
                     &lt;DESCRIPTION>GPRS&lt;/DESCRIPTION>
                     &lt;PRODUCT_STATUS>A&lt;/PRODUCT_STATUS>
                     &lt;PRODUCT_NUMBER>NR-91501420&lt;/PRODUCT_NUMBER>
                     &lt;PRODUCT_SERIAL_NUMBER>SN-91501420&lt;/PRODUCT_SERIAL_NUMBER>
                  &lt;/PRODUCT_DEFINITION>
                  &lt;PRODUCT_PRICE_INFO>
                     &lt;PRODUCT_PRICE_NAME>DATA SEVICE&lt;/PRODUCT_PRICE_NAME>
                     &lt;PRODUCT_PRICE_DESCR>DATA SEVICE&lt;/PRODUCT_PRICE_DESCR>
                     &lt;UOM_TYPE_ID>3&lt;/UOM_TYPE_ID>
                     &lt;TARIFF_ID>111305&lt;/TARIFF_ID>
                  &lt;/PRODUCT_PRICE_INFO>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>msisdn&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>${msisdn}&lt;/PRODUCT_CHAR_VALUE>
                     &lt;DESCRIPTION>MSISDN&lt;/DESCRIPTION>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>1&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>1&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
                  &lt;PRODUCT_CHAR_VALUE>
                     &lt;PRODUCT_CHAR_VALUE_NAME>imsi&lt;/PRODUCT_CHAR_VALUE_NAME>
                     &lt;PRODUCT_CHAR_VALUE>${imsi}&lt;/PRODUCT_CHAR_VALUE>
                     &lt;DESCRIPTION>IMSI&lt;/DESCRIPTION>
                     &lt;BILLING_INDICATOR>1&lt;/BILLING_INDICATOR>
                     &lt;RESOURCE_INDICATOR>1&lt;/RESOURCE_INDICATOR>
                     &lt;PROVISIONING_INDICATOR>1&lt;/PROVISIONING_INDICATOR>
                  &lt;/PRODUCT_CHAR_VALUE>
               &lt;/PRODUCT>
               &lt;OFFER>
                  &lt;PRODUCT_OFFER_KEY>
                     &lt;PRODUCT_OFFER_ID>420&lt;/PRODUCT_OFFER_ID>
                  &lt;/PRODUCT_OFFER_KEY>
                  &lt;PRODUCT_OFFER_DEFINITION>
                     &lt;OFFER_NAME>Texos&lt;/OFFER_NAME>
                     &lt;OFFER_STATUS>A&lt;/OFFER_STATUS>
                     &lt;IS_BUNDLE>0&lt;/IS_BUNDLE>
                     &lt;OFFER_TYPE>POFFR&lt;/OFFER_TYPE>
                  &lt;/PRODUCT_OFFER_DEFINITION>
               &lt;/OFFER>
               &lt;QUANTITY>1&lt;/QUANTITY>
            &lt;/SALES_ORDER_ITEM>
              
              
              
              
            &lt;BILLING_INFO>
               &lt;BILLING_ACCOUNT_KEY>
                  &lt;BILLING_ACCOUNT_ID>${billAcctID}&lt;/BILLING_ACCOUNT_ID>
               &lt;/BILLING_ACCOUNT_KEY>
            &lt;/BILLING_INFO>
            &lt;SUBSCRIPTION_KEY>
               &lt;SUBSCRIPTION_ID>${subID}&lt;/SUBSCRIPTION_ID>
            &lt;/SUBSCRIPTION_KEY>
            &lt;ORDER_PAYMENT>
               &lt;PAYMENT_TYPE>1&lt;/PAYMENT_TYPE>
               &lt;NET_AMOUNT>0&lt;/NET_AMOUNT>
               &lt;GROSS_AMOUNT>0&lt;/GROSS_AMOUNT>
               &lt;PAYMENT_DATE>${nsoDate}&lt;/PAYMENT_DATE>
            &lt;/ORDER_PAYMENT>
         &lt;/pSalesOrder>
        
        
        
        
        
        
      &lt;/aom:NewSalesOrder>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>NewSalesOrder</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.imsi</defaultValue>
      <description></description>
      <id>801188f0-7165-437f-a98e-77fb1337d0ba</id>
      <masked>false</masked>
      <name>imsi</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.msisdn</defaultValue>
      <description></description>
      <id>bb52f552-5cb0-4a60-8d99-ffeab71ee0b5</id>
      <masked>false</masked>
      <name>msisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>ac6ad694-a232-4236-9317-1d33dc51b89a</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.voiceUsageProductID</defaultValue>
      <description></description>
      <id>8b8c3cd2-9172-45ac-8d70-5415fdda30aa</id>
      <masked>false</masked>
      <name>voiceUsageProductID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.voiceFUProductID</defaultValue>
      <description></description>
      <id>434c288e-385e-479e-bce6-1e634f124e58</id>
      <masked>false</masked>
      <name>voiceFUProductID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.smsUsageProductID</defaultValue>
      <description></description>
      <id>20914a12-fae7-4884-b5dd-9c7d1c4df849</id>
      <masked>false</masked>
      <name>smsUsageProductID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.dataUsageProductID</defaultValue>
      <description></description>
      <id>3e6e4cb3-9d3a-4849-b78a-bc1414364af2</id>
      <masked>false</masked>
      <name>dataUsageProductID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.billAcctID</defaultValue>
      <description></description>
      <id>e3af302f-1072-435c-9b02-cd4ee8a2f250</id>
      <masked>false</masked>
      <name>billAcctID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.subID</defaultValue>
      <description></description>
      <id>9f9bdda5-e50e-470b-bbe6-fa195003bc9c</id>
      <masked>false</masked>
      <name>subID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.nsoDate</defaultValue>
      <description></description>
      <id>4d60a76b-8804-48ef-bef7-3f7fba135a32</id>
      <masked>false</masked>
      <name>nsoDate</name>
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
   <wsdlAddress>http://172.30.10.32:8181/AOMWSProduct/AOMWSProduct?wsdl</wsdlAddress>
</WebServiceRequestEntity>
